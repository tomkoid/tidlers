use anyhow::{Context, Result};
use bytes::Bytes;
use futures::stream::{self, StreamExt};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::path::PathBuf;
use std::sync::Arc;
use tidlers::client::{
    TidalClient,
    models::track::{ManifestType, Track, TrackPlaybackInfoPostPaywallResponse},
};

/// struct for handling all download operations
pub struct Downloader {
    output_dir: PathBuf,
    http_client: reqwest::Client,
    max_parallel: usize,
}

impl Downloader {
    pub fn new(output_dir: PathBuf, max_parallel: usize) -> Self {
        Self {
            output_dir,
            http_client: reqwest::Client::new(),
            max_parallel,
        }
    }

    pub async fn download_track(&self, client: &mut TidalClient, track_id: &str) -> Result<()> {
        let track = client
            .get_track(track_id.to_string())
            .await
            .context("Failed to get track info")?;

        println!("track: {}", track.title);
        println!("artist: {}", track.artist.name);
        println!("album: {}", track.album.title);

        let playback_info = client
            .get_track_postpaywall_playback_info(track_id.to_string())
            .await
            .context("Failed to get playback info")?;

        self.download_track_with_info(&track, &playback_info, None, &self.output_dir)
            .await
    }

    pub async fn download_album(&self, client: &mut TidalClient, album_id: &str) -> Result<()> {
        let album = client
            .get_album(album_id.to_string())
            .await
            .context("Failed to get album info")?;

        println!("album: {}", album.title);
        println!("artist: {}", album.artist.name);
        println!("tracks: {}", album.number_of_tracks);

        let album_dir = self.output_dir.join(sanitize_filename::sanitize(format!(
            "{} - {}",
            album.artist.name, album.title
        )));
        std::fs::create_dir_all(&album_dir).context("Failed to create album directory")?;

        // fetch all tracks from the album (handles pagination)
        let mut all_tracks = Vec::new();
        let mut offset = 0;
        let limit = 100;

        loop {
            let items = client
                .get_album_items(album_id.to_string(), Some(limit), Some(offset))
                .await
                .context("Failed to get album tracks")?;

            for item in items.items {
                all_tracks.push(item.item);
            }

            if all_tracks.len() >= items.total_number_of_items as usize {
                break;
            }
            offset += limit;
        }

        println!(
            "\ndownloading {} tracks in parallel (max {})...\n",
            all_tracks.len(),
            self.max_parallel
        );

        // download all tracks in parallel using streams
        // buffer_unordered limits concurrent downloads to max_parallel
        let multi_progress = Arc::new(MultiProgress::new());
        let downloader = Arc::new(self);
        let client = Arc::new(tokio::sync::Mutex::new(client));

        stream::iter(all_tracks)
            .map(|track| {
                let downloader = Arc::clone(&downloader);
                let client = Arc::clone(&client);
                let multi_progress = Arc::clone(&multi_progress);
                let album_dir = album_dir.clone();

                async move {
                    let pb = multi_progress.add(ProgressBar::new(100));
                    pb.set_style(
                        ProgressStyle::default_bar()
                            .template("{msg} [{bar:40.cyan/blue}] {percent}%")
                            .unwrap()
                            .progress_chars("#>-"),
                    );
                    pb.set_message(format!("{:02} - {}", track.track_number, track.title));

                    let track_id = track.id.to_string();
                    let result = {
                        let mut client_guard = client.lock().await;
                        client_guard
                            .get_track_postpaywall_playback_info(track_id)
                            .await
                    };

                    match result {
                        Ok(playback_info) => {
                            let result = downloader
                                .download_track_with_info(
                                    &track,
                                    &playback_info,
                                    Some(pb.clone()),
                                    &album_dir,
                                )
                                .await;
                            pb.finish_with_message(format!(
                                " {:02} - {}",
                                track.track_number, track.title
                            ));
                            result
                        }
                        Err(e) => {
                            pb.finish_with_message(format!(
                                " {:02} - {} ({})",
                                track.track_number, track.title, e
                            ));
                            Err(e.into())
                        }
                    }
                }
            })
            .buffer_unordered(self.max_parallel)
            .collect::<Vec<_>>()
            .await;

        Ok(())
    }

    pub async fn download_playlist(
        &self,
        _client: &mut TidalClient,
        playlist_id: &str,
    ) -> Result<()> {
        println!(
            "playlist download not yet implemented (ID: {})",
            playlist_id
        );
        println!("you can implement this by fetching playlist tracks similar to albums");
        Ok(())
    }

    async fn download_track_with_info(
        &self,
        track: &Track,
        playback_info: &TrackPlaybackInfoPostPaywallResponse,
        progress_bar: Option<ProgressBar>,
        output_dir: &PathBuf,
    ) -> Result<()> {
        let file_name = format!(
            "{:02} - {}.{}",
            track.track_number,
            sanitize_filename::sanitize(&track.title),
            self.get_file_extension(playback_info)
        );

        let output_path = output_dir.join(&file_name);

        // Skip if already exists
        if output_path.exists() {
            if let Some(pb) = progress_bar {
                pb.finish_with_message(format!(
                    " {:02} - {} (exists)",
                    track.track_number, track.title
                ));
            }
            return Ok(());
        }

        match &playback_info.manifest_parsed {
            Some(ManifestType::Dash(dash)) => {
                self.download_dash_track(dash, &output_path, progress_bar)
                    .await?;
            }
            Some(ManifestType::Json(json_manifest)) => {
                if let Some(url) = json_manifest.urls.first() {
                    self.download_file(url, &output_path, progress_bar).await?;
                } else {
                    anyhow::bail!("No URLs in manifest");
                }
            }
            None => {
                anyhow::bail!("No parsed manifest available");
            }
        }

        Ok(())
    }

    async fn download_dash_track(
        &self,
        dash: &tidlers::client::models::track::DashManifest,
        output_path: &PathBuf,
        progress_bar: Option<ProgressBar>,
    ) -> Result<()> {
        let mut combined_data = Vec::new();

        // Step 1: Download initialization segment (required for DASH)
        if let Some(init_url) = dash.get_init_url() {
            let init_data = self.download_segment(init_url).await?;
            combined_data.extend_from_slice(&init_data);
            if let Some(ref pb) = progress_bar {
                pb.set_position(5);
            }
        } else {
            anyhow::bail!("No initialization segment found");
        }

        // Step 2: Download media segments sequentially until we hit 3 consecutive failures
        let mut segment_num = 1;
        let mut consecutive_failures = 0;

        loop {
            if let Some(segment_url) = dash.get_segment_url(segment_num) {
                match self.download_segment(&segment_url).await {
                    Ok(segment_data) => {
                        combined_data.extend_from_slice(&segment_data);
                        consecutive_failures = 0;

                        // Update progress (rough estimate based on segments)
                        if let Some(ref pb) = progress_bar {
                            let progress = 5 + (segment_num as u64 * 95 / 200).min(95);
                            pb.set_position(progress);
                        }
                    }
                    Err(_) => {
                        consecutive_failures += 1;
                        if consecutive_failures >= 3 {
                            break;
                        }
                    }
                }
            } else {
                break;
            }
            segment_num += 1;
        }

        // Write to file
        std::fs::write(output_path, combined_data).context("Failed to write file")?;

        if let Some(ref pb) = progress_bar {
            pb.set_position(100);
        }

        Ok(())
    }

    async fn download_file(
        &self,
        url: &str,
        output_path: &PathBuf,
        progress_bar: Option<ProgressBar>,
    ) -> Result<()> {
        let data = self.download_segment(url).await?;
        std::fs::write(output_path, data).context("Failed to write file")?;

        if let Some(ref pb) = progress_bar {
            pb.set_position(100);
        }

        Ok(())
    }

    async fn download_segment(&self, url: &str) -> Result<Bytes> {
        let response = self
            .http_client
            .get(url)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
            .context("Failed to send request")?;

        if !response.status().is_success() {
            anyhow::bail!("HTTP {}", response.status());
        }

        response.bytes().await.context("Failed to read bytes")
    }

    fn get_file_extension(&self, playback_info: &TrackPlaybackInfoPostPaywallResponse) -> &str {
        // Determine file extension based on manifest type and MIME type
        match &playback_info.manifest_parsed {
            Some(ManifestType::Dash(_)) => "m4a", // HiRes uses fragmented MP4 (m4a)
            Some(ManifestType::Json(json)) => {
                // Standard qualities - check MIME type
                if json.mime_type.contains("flac") {
                    "flac"
                } else if json.mime_type.contains("mp4") || json.mime_type.contains("m4a") {
                    "m4a"
                } else {
                    "m4a"
                }
            }
            None => "m4a",
        }
    }
}
