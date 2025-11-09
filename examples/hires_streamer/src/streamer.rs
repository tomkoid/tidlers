use bytes::Bytes;
use color_eyre::eyre::{Result, eyre};
use rodio::{Decoder, OutputStream, Sink};
use std::io::BufReader;
use tidlers::client::models::track::{
    DashManifest, ManifestType, TrackPlaybackInfoPostPaywallResponse,
};

pub struct DashStreamer {
    http_client: reqwest::Client,
}

impl DashStreamer {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
        }
    }

    async fn download_segment(&self, url: &str) -> Result<Bytes> {
        let url_display = if url.len() > 100 {
            format!("{}...", &url[..100])
        } else {
            url.to_string()
        };
        println!("  Downloading: {}", url_display);

        let response = self
            .http_client
            .get(url)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
            .map_err(|e| eyre!("Failed to download segment: {}", e))?;

        if !response.status().is_success() {
            return Err(eyre!(
                "Failed to download segment: HTTP {}",
                response.status()
            ));
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| eyre!("Failed to read segment bytes: {}", e))?;

        if bytes.is_empty() {
            return Err(eyre!("Downloaded segment is empty"));
        }

        println!("  Downloaded {} bytes", bytes.len());

        Ok(bytes)
    }

    async fn download_and_combine_segments(
        &self,
        dash: &DashManifest,
        max_segments: Option<u32>,
    ) -> Result<Vec<u8>> {
        let mut combined_data = Vec::new();

        // Download initialization segment first
        if let Some(init_url) = dash.get_init_url() {
            println!("Downloading initialization segment...");
            let init_data = self
                .download_segment(init_url)
                .await
                .map_err(|e| eyre!("Failed to download init segment: {}", e))?;
            println!("  Init segment size: {} bytes", init_data.len());
            combined_data.extend_from_slice(&init_data);
        } else {
            return Err(eyre!("No initialization segment found"));
        }

        // Determine how many segments to download
        let segment_count = max_segments.unwrap_or(999);

        println!("Downloading up to {} media segments...", segment_count);
        let mut segments_downloaded = 0;
        let mut consecutive_failures = 0;

        for segment_num in 1..=segment_count {
            if let Some(segment_url) = dash.get_segment_url(segment_num) {
                match self.download_segment(&segment_url).await {
                    Ok(segment_data) => {
                        combined_data.extend_from_slice(&segment_data);
                        segments_downloaded += 1;
                        consecutive_failures = 0;
                    }
                    Err(e) => {
                        consecutive_failures += 1;
                        if consecutive_failures >= 3 {
                            println!(
                                "Stopped after {} consecutive failures",
                                consecutive_failures
                            );
                            break;
                        }
                        println!(
                            "  Warning: Segment {} failed ({}), trying next...",
                            segment_num, e
                        );
                    }
                }
            } else {
                println!("No more segments available");
                break;
            }
        }

        if segments_downloaded == 0 {
            return Err(eyre!("No media segments were downloaded"));
        }

        println!(
            "Total: {} segments, {} bytes",
            segments_downloaded,
            combined_data.len()
        );

        // Save to file for inspection/debugging
        let debug_path = "tidlers_dash_combined.mp4";
        if let Err(e) = std::fs::write(debug_path, &combined_data) {
            println!("Warning: Could not save debug file: {}", e);
        } else {
            println!("Debug: Saved combined data to {}", debug_path);
        }

        Ok(combined_data)
    }

    pub async fn stream_track(
        &self,
        playback_info: &TrackPlaybackInfoPostPaywallResponse,
        max_segments: Option<u32>,
    ) -> Result<()> {
        let manifest = playback_info
            .manifest_parsed
            .as_ref()
            .ok_or_else(|| eyre!("No parsed manifest available"))?;

        match manifest {
            ManifestType::Dash(dash) => {
                println!("\n=== DASH Manifest Info ===");
                println!("MIME Type: {}", dash.mime_type);
                println!("Codecs: {}", dash.codecs);
                if let Some(bitrate) = dash.bitrate {
                    println!(
                        "Bitrate: {} bps ({:.2} Mbps)",
                        bitrate,
                        bitrate as f64 / 1_000_000.0
                    );
                }
                println!("========================\n");

                // Download and combine segments
                println!("Starting download...");
                let audio_data = self
                    .download_and_combine_segments(dash, max_segments)
                    .await?;

                // Play the audio
                println!("\nInitializing audio playback...");
                self.play_audio_data(audio_data)?;
            }
            ManifestType::Json(json_manifest) => {
                println!("\n=== JSON Manifest Info ===");
                println!("MIME Type: {}", json_manifest.mime_type);
                println!("Codecs: {}", json_manifest.codecs);
                println!("Encryption: {}", json_manifest.encryption_type);
                println!("URLs: {} available", json_manifest.urls.len());
                println!("=========================\n");

                // For JSON manifest (non-HiRes), just download the single URL
                if let Some(url) = json_manifest.urls.first() {
                    println!("Downloading audio file...");
                    let audio_data = self.download_segment(url).await?;

                    println!("\nInitializing audio playback...");
                    self.play_audio_data(audio_data.to_vec())?;
                } else {
                    return Err(eyre!("No URLs in manifest"));
                }
            }
        }

        Ok(())
    }

    fn play_audio_data(&self, audio_data: Vec<u8>) -> Result<()> {
        if audio_data.is_empty() {
            return Err(eyre!("Cannot play empty audio data"));
        }

        println!("Preparing audio ({} bytes)...", audio_data.len());

        // Save to both temp file and permanent debug file
        let temp_path = std::env::temp_dir().join("tidlers_temp_audio.mp4");
        let debug_path = "tidlers_playback.mp4";

        std::fs::write(&temp_path, &audio_data)
            .map_err(|e| eyre!("Failed to write temp file: {}", e))?;
        std::fs::write(debug_path, &audio_data)
            .map_err(|e| eyre!("Failed to write debug file: {}", e))?;

        println!("Saved audio to: {}", debug_path);
        println!("Attempting playback...");

        // Try to play with rodio - this may fail for fragmented MP4s
        let playback_result = self.try_play_file(&temp_path);

        // Clean up temp file
        let _ = std::fs::remove_file(&temp_path);

        match playback_result {
            Ok(_) => {
                println!("\nPlayback finished!");
                Ok(())
            }
            Err(e) => {
                println!("\n{}", "=".repeat(70));
                println!("Rodio playback failed: {}", e);
                println!("{}", "=".repeat(70));
                println!(
                    "\nHowever, the audio file has been saved to: {}",
                    debug_path
                );
                println!("\nYou can play it with:");
                println!("  ffplay {}", debug_path);
                println!("  mpv {}", debug_path);
                println!("  vlc {}", debug_path);
                println!("\nAlternatively, use quality levels 1-3 (Low/High/Lossless)");
                println!("which work perfectly with direct playback.");
                println!("{}", "=".repeat(70));

                Err(eyre!(
                    "HiRes playback streaming not supported. File saved to: {}",
                    debug_path
                ))
            }
        }
    }

    fn try_play_file(&self, path: &std::path::Path) -> Result<()> {
        // Catch panics from rodio/symphonia
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<()> {
            let (_stream, stream_handle) = OutputStream::try_default()
                .map_err(|e| eyre!("Failed to open audio output device: {}", e))?;

            let sink = Sink::try_new(&stream_handle)
                .map_err(|e| eyre!("Failed to create audio sink: {}", e))?;

            let file =
                std::fs::File::open(path).map_err(|e| eyre!("Failed to open file: {}", e))?;

            let source = Decoder::new(BufReader::new(file))
                .map_err(|e| eyre!("Failed to decode audio: {}", e))?;

            println!("Successfully decoded! Playing audio... (press Ctrl+C to stop)");
            sink.append(source);
            sink.sleep_until_end();

            Ok(())
        }));

        match result {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => Err(e),
            Err(panic_err) => {
                if let Some(s) = panic_err.downcast_ref::<&str>() {
                    Err(eyre!("Decoder panicked: {}", s))
                } else if let Some(s) = panic_err.downcast_ref::<String>() {
                    Err(eyre!("Decoder panicked: {}", s))
                } else {
                    Err(eyre!("Decoder panicked with unknown error"))
                }
            }
        }
    }
}
