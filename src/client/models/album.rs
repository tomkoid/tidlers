use serde::{Deserialize, Serialize};

use crate::client::models::{ArtistNameId, artist::Artist, media::MediaMetadata, track::Track};

/// Used generically to represent an album in various responses
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub id: i64,
    pub title: String,
    pub cover: Option<String>,
    pub vibrant_color: Option<String>,
    pub video_cover: Option<String>,
    pub release_date: Option<String>,
}

/// Response from TIDAL when requesting album info
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumResponse {
    pub id: i64,
    pub title: String,
    pub duration: u64,
    pub stream_ready: bool,
    pub pay_to_stream: bool,
    pub ad_supported_stream_ready: bool,
    pub dj_ready: bool,
    pub stem_ready: bool,
    pub stream_start_date: String,
    pub allow_streaming: bool,
    pub premium_streaming_only: bool,
    pub number_of_tracks: u32,
    pub number_of_videos: u32,
    pub number_of_volumes: u32,
    pub release_date: String,
    pub copyright: String,
    #[serde(rename = "type")]
    pub album_type: String,
    pub version: Option<String>,
    pub url: String,
    pub cover: String,
    pub vibrant_color: Option<String>,
    pub video_cover: Option<String>,
    pub explicit: bool,
    pub upc: String,
    pub popularity: u32,
    pub audio_quality: String,
    pub audio_modes: Vec<String>,
    /// The tiers this release is actually available in. `audio_quality` above
    /// names one tier and is not always the one that streams, so a client that
    /// wants to describe a release reads this.
    #[serde(default)]
    pub media_metadata: MediaMetadata,
    pub upload: bool,
    pub artist: Artist,
    pub artists: Vec<Artist>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistAlbum {
    pub id: i64,
    pub title: String,
    pub duration: u64,
    pub stream_ready: bool,
    pub pay_to_stream: bool,
    pub ad_supported_stream_ready: bool,
    pub dj_ready: bool,
    pub stem_ready: bool,
    pub stream_start_date: String,
    pub allow_streaming: bool,
    pub premium_streaming_only: bool,
    pub number_of_tracks: u32,
    pub number_of_videos: u32,
    pub number_of_volumes: u32,
    pub release_date: String,
    pub copyright: String,
    #[serde(rename = "type")]
    pub album_type: String,
    pub version: Option<String>,
    pub url: String,
    pub cover: String,
    pub vibrant_color: Option<String>,
    pub video_cover: Option<String>,
    pub explicit: bool,
    pub upc: String,
    pub popularity: u32,
    pub audio_quality: String,
    pub audio_modes: Vec<String>,
    #[serde(default)]
    pub media_metadata: MediaMetadata,
    pub upload: bool,
    pub artist: Artist,
    pub artists: Vec<Artist>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumItemsResponse {
    pub limit: i32,
    pub offset: i32,
    pub total_number_of_items: i32,
    pub items: Vec<AlbumItemsEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumItemsEntry {
    pub item: Track,
    #[serde(rename = "type")]
    pub album_type: String,
}

impl Album {
    pub fn get_cover_url(&self, size_x: u32, size_y: u32) -> Option<String> {
        // split string by dashes
        let cover = self.cover.clone()?;
        let cover_parts: Vec<&str> = cover.split('-').collect();

        let mut cover_path = String::new();
        for part in cover_parts.iter() {
            cover_path.push_str(part);
        }

        let size = format!("{}x{}", size_x, size_y);
        Some(format!(
            "https://resources.tidal.com/images/{}/{}",
            cover_path, size
        ))
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralCredit {
    #[serde(rename = "type")]
    pub credit_type: String,
    pub contributors: Vec<ArtistNameId>,
}

pub type GeneralCreditsResponse = Vec<GeneralCredit>;
pub type AlbumCreditsResponse = Vec<GeneralCredit>; // for backwards compat

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumItemsWithCreditsEntry {
    pub item: Track,
    #[serde(rename = "type")]
    pub item_type: String,
    pub credits: Vec<GeneralCredit>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumItemsWithCreditsResponse {
    pub limit: u32,
    pub offset: u32,
    pub total_number_of_items: u32,
    pub items: Vec<AlbumItemsWithCreditsEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumReviewResponse {
    pub source: String,
    pub last_updated: String,
    pub text: String,
}

#[cfg(test)]
mod tests {
    use super::AlbumResponse;

    /// The fields an album response always carries, as a JSON literal the
    /// tests below extend.
    fn album_json(extra: &str) -> String {
        format!(
            r#"{{
                "id": 9621920, "title": "Missa in memoriam", "duration": 1758,
                "streamReady": true, "payToStream": false, "adSupportedStreamReady": true,
                "djReady": true, "stemReady": false, "streamStartDate": "2010-01-01T00:00:00.000+0000",
                "allowStreaming": true, "premiumStreamingOnly": false,
                "numberOfTracks": 5, "numberOfVideos": 0, "numberOfVolumes": 1,
                "releaseDate": "2006-01-01", "copyright": "c", "type": "ALBUM",
                "version": null, "url": "https://tidal.com/album/9621920", "cover": "abc",
                "vibrantColor": null, "videoCover": null, "explicit": false, "upc": "1",
                "popularity": 1, "audioQuality": "HIGH", "audioModes": ["STEREO"],
                "upload": false,
                "artist": {{ "id": 1, "name": "Arrigo Barnabé", "type": "MAIN" }},
                "artists": [{{ "id": 1, "name": "Arrigo Barnabé", "type": "MAIN" }}]
                {extra}
            }}"#
        )
    }

    #[test]
    fn reads_the_advertised_tiers() {
        let json = album_json(r#", "mediaMetadata": { "tags": ["LOSSLESS", "HIRES_LOSSLESS"] }"#);
        let album: AlbumResponse = serde_json::from_str(&json).expect("album should parse");
        assert_eq!(album.media_metadata.tags, ["LOSSLESS", "HIRES_LOSSLESS"]);
    }

    #[test]
    fn an_album_without_media_metadata_advertises_nothing() {
        let album: AlbumResponse =
            serde_json::from_str(&album_json("")).expect("album should parse");
        assert!(album.media_metadata.tags.is_empty());
    }
}
