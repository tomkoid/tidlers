use std::fmt::Display;

use serde::{Deserialize, Deserializer, Serialize, de};

use crate::client::models::collection::CollectionCreator;
use crate::client::models::track::Track;

/// Response containing playlist items with pagination
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItemsResponse {
    pub limit: u64,
    pub offset: u64,
    pub total_number_of_items: u64,
    pub items: Vec<PlaylistItem>,
}

#[derive(Debug)]
pub struct PlaylistItemsWithEtag {
    pub items: PlaylistItemsResponse,
    pub etag: String,
}

#[derive(Debug, Clone)]
pub enum PlaylistItemsOrder {
    Index,
    Date,
    Name,
    Album,
    Artist,
}

impl Display for PlaylistItemsOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PlaylistItemsOrder::Index => "INDEX".to_string(),
                PlaylistItemsOrder::Date => "DATE".to_string(),
                PlaylistItemsOrder::Name => "NAME".to_string(),
                PlaylistItemsOrder::Album => "ALBUM".to_string(),
                PlaylistItemsOrder::Artist => "ARTIST".to_string(),
            }
        )
    }
}

/// Represents a single item in a playlist.
///
/// Playlist video payloads can have a null singular `artist`. Deserialization
/// repairs that playlist-only shape from `artists[0]`, or uses an explicit
/// unknown-artist sentinel when TIDAL supplies neither. The global Track model
/// remains strict for endpoints that guarantee a primary artist.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItem {
    pub item: Track,
    #[serde(rename = "type")]
    pub item_type: String,
    pub cut: Option<serde_json::Value>,
}

impl<'de> Deserialize<'de> for PlaylistItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Wire {
            item: serde_json::Value,
            #[serde(rename = "type")]
            item_type: String,
            cut: Option<serde_json::Value>,
        }

        let Wire {
            mut item,
            item_type,
            cut,
        } = Wire::deserialize(deserializer)?;
        let object = item
            .as_object_mut()
            .ok_or_else(|| de::Error::custom("playlist item payload is not an object"))?;
        let repair_artist = match object.get("artist") {
            None | Some(serde_json::Value::Null) => true,
            Some(serde_json::Value::Object(artist)) => {
                !artist.get("name").is_some_and(serde_json::Value::is_string)
            }
            Some(_) => false,
        };
        if repair_artist {
            let fallback = object
                .get("artists")
                .and_then(serde_json::Value::as_array)
                .and_then(|artists| artists.first())
                .filter(|artist| artist.get("name").is_some_and(serde_json::Value::is_string))
                .cloned();
            let artist = fallback.unwrap_or_else(|| match object.get("artist") {
                Some(serde_json::Value::Object(artist)) => {
                    let mut artist = artist.clone();
                    artist.insert(
                        "name".into(),
                        serde_json::Value::String("Unknown Artist".into()),
                    );
                    serde_json::Value::Object(artist)
                }
                _ => serde_json::json!({"id": 0, "name": "Unknown Artist"}),
            });
            object.insert("artist".into(), artist);
        }
        object
            .entry("artists")
            .or_insert_with(|| serde_json::Value::Array(Vec::new()));
        let item = serde_json::from_value(item).map_err(de::Error::custom)?;
        Ok(Self {
            item,
            item_type,
            cut,
        })
    }
}

/// Response containing a list of playlists
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPlaylistsResponse {
    pub items: Vec<PlaylistResponse>,
}

/// Detailed information about a playlist
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PlaylistResponse {
    pub uuid: String,
    pub title: String,
    #[serde(rename = "numberOfTracks")]
    pub number_of_tracks: u64,
    #[serde(rename = "numberOfVideos")]
    pub number_of_videos: u64,
    pub creator: PlaylistCreator,
    pub description: String,
    pub duration: u64,
    #[serde(rename = "lastUpdated")]
    pub last_updated: String,
    pub created: String,
    #[serde(rename = "type")]
    pub playlist_type: String,
    #[serde(rename = "publicPlaylist")]
    pub public_playlist: bool,
    pub url: String,
    pub image: String,
    pub popularity: u64,
    #[serde(rename = "squareImage")]
    pub square_image: String,
    #[serde(rename = "customImageUrl")]
    pub custom_image_url: Option<String>,
    #[serde(rename = "promotedArtists")]
    pub promoted_artists: Vec<String>,
    #[serde(rename = "lastItemAddedAt")]
    pub last_item_added_at: Option<String>,
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PlaylistCreator {
    pub id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicUserPlaylistsResponse {
    pub items: Vec<PublicUserPlaylistItem>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicUserPlaylistItem {
    pub playlist: PublicUserPlaylist,
    pub follow_info: FollowInfo,
    pub profile: Profile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicUserPlaylist {
    pub uuid: String,
    #[serde(rename = "type")]
    pub playlist_type: String,
    pub creator: CollectionCreator,
    pub curators: Vec<Curator>,
    pub content_behavior: String,
    pub sharing_level: String,
    pub status: String,
    pub source: String,
    pub title: String,
    pub description: String,
    pub image: String,
    pub square_image: String,
    pub custom_image_url: Option<String>,
    pub url: String,
    pub created: String,
    pub last_updated: String,
    pub last_item_added_at: Option<String>,
    pub duration: i64,
    pub number_of_tracks: i64,
    pub number_of_videos: i64,
    pub promoted_artists: Vec<serde_json::Value>,
    pub trn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Curator {
    pub id: i64,
    pub name: String,
    pub handle: Option<String>,
    pub picture: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowInfo {
    pub nr_of_followers: i64,
    pub tidal_resource_name: String,
    pub followed: bool,
    pub follow_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub user_id: i64,
    pub name: String,
    pub color: Vec<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::PlaylistItemsResponse;
    use serde_json::{Value, json};

    fn base(id: u64, title: &str) -> Value {
        json!({
            "id": id, "title": title, "duration": 240, "replayGain": -3.0,
            "peak": 0.9, "allowStreaming": true, "streamReady": true,
            "payToStream": false, "adSupportedStreamReady": false,
            "djReady": false, "stemReady": false, "premiumStreamingOnly": false,
            "trackNumber": 1, "volumeNumber": 1, "version": null,
            "popularity": 1, "copyright": null, "bpm": null, "key": null,
            "keyScale": null, "url": "https://example.invalid/media",
            "isrc": null, "editable": false, "explicit": false,
            "audioQuality": "HIGH", "audioModes": ["STEREO"],
            "mediaMetadata": null, "upload": false, "accessType": "PUBLIC",
            "spotlighted": false, "dateAdded": null, "index": 0,
            "artists": [], "album": null, "mixes": null, "itemUuid": null
        })
    }

    fn response(items: Vec<Value>) -> PlaylistItemsResponse {
        serde_json::from_value(json!({
            "limit": 100, "offset": 0,
            "totalNumberOfItems": items.len(), "items": items
        }))
        .expect("playlist items should parse")
    }

    #[test]
    fn video_with_null_artist_uses_the_artists_list_and_own_artwork() {
        let mut video = base(456, "A Music Video");
        video["artist"] = Value::Null;
        video["artists"] = json!([{"id": 7, "name": "Video Artist"}]);
        video["imageId"] = json!("7bd9a4c2-424a-49cf-afd9-31f6e526a71e");
        let page = response(vec![json!({"item": video, "type": "video", "cut": null})]);
        let item = &page.items[0];
        assert_eq!(item.item_type, "video");
        assert_eq!(item.item.artist.name, "Video Artist");
        assert!(item.item.album.is_none());
        assert_eq!(
            item.item.image_id.as_deref(),
            Some("7bd9a4c2-424a-49cf-afd9-31f6e526a71e")
        );
    }

    #[test]
    fn null_artist_name_falls_back_to_the_artists_list() {
        let mut video = base(654, "Null-name Video");
        video["artist"] = json!({"id": 99, "name": null});
        video["artists"] = json!([{"id": 7, "name": "Video Artist"}]);
        let page = response(vec![json!({"item": video, "type": "video"})]);
        assert_eq!(page.items[0].item.artist.id, 7);
        assert_eq!(page.items[0].item.artist.name, "Video Artist");
    }

    #[test]
    fn null_artist_name_without_a_list_preserves_id_and_uses_unknown_name() {
        let mut video = base(655, "Null-name Video");
        video["artist"] = json!({"id": 99, "name": null});
        video.as_object_mut().unwrap().remove("artists");
        let page = response(vec![json!({"item": video, "type": "video"})]);
        assert_eq!(page.items[0].item.artist.id, 99);
        assert_eq!(page.items[0].item.artist.name, "Unknown Artist");
    }

    #[test]
    fn artistless_video_uses_an_explicit_unknown_sentinel() {
        let mut video = base(789, "Artist-less Video");
        video["artist"] = Value::Null;
        video.as_object_mut().unwrap().remove("artists");
        let page = response(vec![json!({"item": video, "type": "video"})]);
        assert_eq!(page.items[0].item.artist.id, 0);
        assert_eq!(page.items[0].item.artist.name, "Unknown Artist");
        assert!(page.items[0].item.artists.is_empty());
    }

    #[test]
    fn regular_track_keeps_its_artist_and_album() {
        let mut track = base(123, "A Song");
        track["artist"] = json!({"id": 1, "name": "An Artist"});
        track["album"] = json!({"id": 9, "title": "An Album", "cover": "ab-cd"});
        let page = response(vec![json!({"item": track, "type": "track"})]);
        let track = &page.items[0].item;
        assert_eq!(track.artist.name, "An Artist");
        assert_eq!(track.album.as_ref().unwrap().title, "An Album");
        assert!(track.image_id.is_none());
    }

    #[test]
    fn malformed_nonnull_artist_still_fails() {
        let mut video = base(456, "Malformed Video");
        video["artist"] = json!("not an artist object");
        video["artists"] = json!([{"id": 7, "name": "Fallback must not mask a malformed artist"}]);
        assert!(
            serde_json::from_value::<PlaylistItemsResponse>(json!({
                "limit": 100, "offset": 0, "totalNumberOfItems": 1,
                "items": [{"item": video, "type": "video"}]
            }))
            .is_err()
        );
    }

    #[test]
    fn malformed_item_shapes_still_fail() {
        for item in [
            json!({"type": "video"}),
            json!({"item": {"id": 1}, "type": "track"}),
            json!({"item": "wrong type", "type": "track"}),
        ] {
            assert!(
                serde_json::from_value::<PlaylistItemsResponse>(json!({
                    "limit": 100, "offset": 0, "totalNumberOfItems": 1,
                    "items": [item]
                }))
                .is_err()
            );
        }
    }
}
