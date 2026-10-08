use std::fmt::Display;

use serde::{Deserialize, Deserializer, Serialize, de};

use crate::client::models::collection::CollectionCreator;
use crate::client::models::track::Track;
use crate::client::models::video::Video;

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
/// TIDAL tags each entry with a `type` next to its `item`, and the two kinds
/// have different shapes: video payloads carry none of the audio fields
/// [`Track`] requires, so [`PlaylistMedia`] decodes each into its own model.
///
/// Playlist entries can also have a null singular `artist`. Deserialization
/// repairs that playlist-only shape from `artists[0]`, or uses an explicit
/// unknown-artist sentinel when TIDAL supplies neither. The Track and Video
/// models remain strict for endpoints that guarantee a primary artist.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistItem {
    #[serde(flatten)]
    pub item: PlaylistMedia,
    pub cut: Option<serde_json::Value>,
}

/// The media of a [`PlaylistItem`], selected by the entry's `type`
/// (`"track"` or `"video"`). Any other type is a deserialization error.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "item", rename_all = "lowercase")]
pub enum PlaylistMedia {
    Track(Track),
    Video(Video),
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
        let item = serde_json::from_value(serde_json::json!({ "type": item_type, "item": item }))
            .map_err(de::Error::custom)?;
        Ok(Self { item, cut })
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
    use super::{PlaylistItem, PlaylistItemsResponse, PlaylistMedia};
    use crate::client::models::{track::Track, video::Video};
    use serde_json::{Value, json};

    /// Captured from TIDAL: a track entry from the mixed "Videos: Best of 2022" playlist.
    const TRACK: &str = r##"{
        "item": {
            "id": 231039906,
            "title": "Blacklight Shine",
            "duration": 176,
            "replayGain": -9.85,
            "peak": 1.0,
            "allowStreaming": true,
            "streamReady": true,
            "payToStream": false,
            "adSupportedStreamReady": true,
            "djReady": true,
            "stemReady": false,
            "streamStartDate": "2022-06-21T00:00:00.000+0000",
            "premiumStreamingOnly": false,
            "trackNumber": 1,
            "volumeNumber": 1,
            "version": null,
            "popularity": 53,
            "copyright": "℗ 2022 Clouds Hill",
            "bpm": 118,
            "key": "F",
            "keyScale": "MINOR",
            "description": null,
            "url": "http://www.tidal.com/track/231039906",
            "isrc": "DELK22100395",
            "editable": false,
            "explicit": false,
            "audioQuality": "LOSSLESS",
            "audioModes": [
                "STEREO"
            ],
            "mediaMetadata": {
                "tags": [
                    "LOSSLESS",
                    "HIRES_LOSSLESS"
                ]
            },
            "upload": false,
            "accessType": "PUBLIC",
            "spotlighted": false,
            "ai": false,
            "artist": {
                "id": 24959,
                "name": "The Mars Volta",
                "handle": null,
                "type": "MAIN",
                "picture": "bb43a530-957f-48b8-8ab2-710500bff8a7"
            },
            "artists": [
                {
                    "id": 24959,
                    "name": "The Mars Volta",
                    "handle": null,
                    "type": "MAIN",
                    "picture": "bb43a530-957f-48b8-8ab2-710500bff8a7"
                }
            ],
            "album": {
                "id": 231039905,
                "title": "Blacklight Shine",
                "cover": "8538e8f2-3e29-4372-95a3-d3e52233a938",
                "vibrantColor": "#c4a25a",
                "videoCover": null,
                "releaseDate": "2022-06-21"
            },
            "mixes": {
                "TRACK_MIX": "001dbef3f7310332d0df126a80864b"
            },
            "dateAdded": "2022-11-29T20:58:16.823+0000",
            "index": 3100000,
            "itemUuid": "b5141c46-e994-40aa-9af1-f14e694b9063"
        },
        "type": "track",
        "cut": null
    }"##;

    /// Captured from TIDAL: a video entry from the same playlist.
    const VIDEO: &str = r##"{
        "item": {
            "id": 245978483,
            "title": "Kill to Be Heard",
            "volumeNumber": 0,
            "trackNumber": 0,
            "releaseDate": "2022-09-01T00:00:00.000+0000",
            "imagePath": null,
            "imageId": "06716ff5-d80f-448c-bf37-916e12ca07e5",
            "vibrantColor": "#30566e",
            "duration": 119,
            "quality": "MP4_1080P",
            "streamReady": true,
            "adSupportedStreamReady": true,
            "djReady": true,
            "stemReady": false,
            "streamStartDate": "2022-09-01T00:00:00.000+0000",
            "allowStreaming": true,
            "explicit": true,
            "popularity": 1,
            "type": "Music Video",
            "adsUrl": null,
            "adsPrePaywallOnly": true,
            "artist": {
                "id": 3970187,
                "name": "OFF!",
                "handle": null,
                "type": "MAIN",
                "picture": "d257ccb0-bce4-41a3-b02a-2dfe03247d34"
            },
            "artists": [
                {
                    "id": 3970187,
                    "name": "OFF!",
                    "handle": null,
                    "type": "MAIN",
                    "picture": "d257ccb0-bce4-41a3-b02a-2dfe03247d34"
                }
            ],
            "album": null,
            "dateAdded": "2022-11-29T20:58:16.823+0000",
            "index": 2900000,
            "itemUuid": "98376c20-44e8-4d4b-ad82-c3a1a18034e6"
        },
        "type": "video",
        "cut": null
    }"##;

    /// Captured from TIDAL: "In Da Club" from "Classic Hip-Hop Videos": null `artist`, empty `artists`.
    const ARTISTLESS_VIDEO: &str = r##"{
        "item": {
            "id": 37690657,
            "title": "In Da Club",
            "volumeNumber": 0,
            "trackNumber": 0,
            "releaseDate": "2004-01-01T00:00:00.000+0000",
            "imagePath": null,
            "imageId": "410fbd9a-1c6e-448a-9c93-094fd99c295c",
            "vibrantColor": "#83403d",
            "duration": 248,
            "quality": "MP4_1080P",
            "streamReady": false,
            "adSupportedStreamReady": false,
            "djReady": false,
            "stemReady": false,
            "streamStartDate": null,
            "allowStreaming": false,
            "explicit": false,
            "popularity": 59,
            "type": "Music Video",
            "adsUrl": null,
            "adsPrePaywallOnly": true,
            "artist": null,
            "artists": [],
            "album": null,
            "dateAdded": "2024-05-17T19:15:36.849+0000",
            "index": 2300000,
            "itemUuid": "80b97b08-79d0-4a13-bbd4-c593044f92d2"
        },
        "type": "video",
        "cut": null
    }"##;

    fn entry(fixture: &str) -> Value {
        serde_json::from_str(fixture).expect("fixture is JSON")
    }

    fn response(items: Vec<Value>) -> Result<PlaylistItemsResponse, serde_json::Error> {
        serde_json::from_value(json!({
            "limit": 100, "offset": 0,
            "totalNumberOfItems": items.len(), "items": items
        }))
    }

    fn parse(item: Value) -> PlaylistItem {
        response(vec![item])
            .expect("playlist items should parse")
            .items
            .remove(0)
    }

    fn track(item: &PlaylistItem) -> &Track {
        match &item.item {
            PlaylistMedia::Track(track) => track,
            other => panic!("expected a track, got {other:?}"),
        }
    }

    fn video(item: &PlaylistItem) -> &Video {
        match &item.item {
            PlaylistMedia::Video(video) => video,
            other => panic!("expected a video, got {other:?}"),
        }
    }

    #[test]
    fn mixed_page_decodes_each_entry_by_its_type() {
        let page = response(vec![entry(TRACK), entry(VIDEO), entry(ARTISTLESS_VIDEO)])
            .expect("a real mixed page should parse");

        let track = track(&page.items[0]);
        assert_eq!(track.title, "Blacklight Shine");
        assert_eq!(track.artist.name, "The Mars Volta");
        assert_eq!(track.album.as_ref().unwrap().title, "Blacklight Shine");
        assert_eq!(track.audio_quality, "LOSSLESS");

        // Video payloads carry no replayGain, url, audioQuality, ... so they
        // could never decode as Track; they keep their own fields instead.
        let video = video(&page.items[1]);
        assert_eq!(video.title, "Kill to Be Heard");
        assert_eq!(video.artist.name, "OFF!");
        assert_eq!(video.quality, "MP4_1080P");
        assert!(video.album.is_none());
        assert_eq!(
            video.image_id.as_deref(),
            Some("06716ff5-d80f-448c-bf37-916e12ca07e5")
        );
        assert_eq!(video.index, Some(2900000));
        assert_eq!(
            video.item_uuid.as_deref(),
            Some("98376c20-44e8-4d4b-ad82-c3a1a18034e6")
        );
        assert_eq!(
            video.date_added.as_deref(),
            Some("2022-11-29T20:58:16.823+0000")
        );
    }

    #[test]
    fn artistless_video_uses_an_explicit_unknown_sentinel() {
        let item = parse(entry(ARTISTLESS_VIDEO));
        let video = video(&item);
        assert_eq!(video.title, "In Da Club");
        assert_eq!(video.artist.id, 0);
        assert_eq!(video.artist.name, "Unknown Artist");
        assert!(video.artists.is_empty());
    }

    #[test]
    fn null_artist_uses_the_artists_list() {
        for fixture in [TRACK, VIDEO] {
            let mut item = entry(fixture);
            item["item"]["artist"] = Value::Null;
            item["item"]["artists"] = json!([{"id": 7, "name": "Listed Artist"}]);
            let item = parse(item);
            let artist = match &item.item {
                PlaylistMedia::Track(track) => &track.artist,
                PlaylistMedia::Video(video) => &video.artist,
            };
            assert_eq!((artist.id, artist.name.as_str()), (7, "Listed Artist"));
        }
    }

    #[test]
    fn null_artist_name_falls_back_to_the_artists_list() {
        let mut item = entry(VIDEO);
        item["item"]["artist"] = json!({"id": 99, "name": null});
        item["item"]["artists"] = json!([{"id": 7, "name": "Listed Artist"}]);
        let item = parse(item);
        assert_eq!(video(&item).artist.id, 7);
        assert_eq!(video(&item).artist.name, "Listed Artist");
    }

    #[test]
    fn null_artist_name_without_a_list_preserves_id_and_uses_unknown_name() {
        let mut item = entry(VIDEO);
        item["item"]["artist"] = json!({"id": 99, "name": null});
        item["item"].as_object_mut().unwrap().remove("artists");
        let item = parse(item);
        assert_eq!(video(&item).artist.id, 99);
        assert_eq!(video(&item).artist.name, "Unknown Artist");
    }

    #[test]
    fn malformed_nonnull_artist_still_fails() {
        let mut item = entry(VIDEO);
        item["item"]["artist"] = json!("not an artist object");
        item["item"]["artists"] =
            json!([{"id": 7, "name": "Fallback must not mask a malformed artist"}]);
        assert!(response(vec![item]).is_err());
    }

    #[test]
    fn entries_must_match_their_declared_type() {
        let mut video_as_track = entry(VIDEO);
        video_as_track["type"] = json!("track");
        let mut track_as_video = entry(TRACK);
        track_as_video["type"] = json!("video");
        let mut unknown = entry(VIDEO);
        unknown["type"] = json!("upload");
        for item in [video_as_track, track_as_video, unknown] {
            assert!(response(vec![item]).is_err());
        }
    }

    #[test]
    fn malformed_item_shapes_still_fail() {
        for item in [
            json!({"type": "video"}),
            json!({"item": {"id": 1}, "type": "track"}),
            json!({"item": "wrong type", "type": "track"}),
        ] {
            assert!(response(vec![item]).is_err());
        }
    }

    #[test]
    fn serialization_keeps_the_wire_shape() {
        for fixture in [TRACK, VIDEO] {
            let item = parse(entry(fixture));
            let wire = serde_json::to_value(&item).expect("serialize");
            assert_eq!(wire["type"], entry(fixture)["type"]);
            assert!(wire["item"].is_object());
            assert!(wire.get("cut").is_some());
            let again: PlaylistItem = serde_json::from_value(wire).expect("round trip");
            assert_eq!(
                std::mem::discriminant(&again.item),
                std::mem::discriminant(&item.item)
            );
        }
    }
}
