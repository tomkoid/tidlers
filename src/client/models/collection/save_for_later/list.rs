use serde::{Deserialize, Deserializer, Serialize};

/// Response containing resources added to saved for later collection
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSaveForLaterResponse {
    pub data: Vec<serde_json::Value>,
    #[serde(default)]
    pub included: Vec<SaveForLaterItemsIncludedResource>,
    #[serde(default)]
    pub links: Option<serde_json::Value>,
}

/// An included resource returned by the save-for-later relationship.
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum SaveForLaterItemsIncludedResource {
    Track(SaveForLaterTrack),
    Album(SaveForLaterAlbum),
    Playlist(SaveForLaterPlaylist),
    Artwork(SaveForLaterArtwork),
    Unknown(SaveForLaterUnknownResource),
}

impl<'de> Deserialize<'de> for SaveForLaterItemsIncludedResource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        let resource_type = value
            .get("type")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| serde::de::Error::missing_field("type"))?;

        match resource_type {
            "tracks" => serde_json::from_value(value)
                .map(Self::Track)
                .map_err(serde::de::Error::custom),
            "albums" => serde_json::from_value(value)
                .map(Self::Album)
                .map_err(serde::de::Error::custom),
            "playlists" => serde_json::from_value(value)
                .map(Self::Playlist)
                .map_err(serde::de::Error::custom),
            "artworks" => serde_json::from_value(value)
                .map(Self::Artwork)
                .map_err(serde::de::Error::custom),
            _ => serde_json::from_value(value)
                .map(Self::Unknown)
                .map_err(serde::de::Error::custom),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveForLaterTrack {
    pub id: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    #[serde(default)]
    pub attributes: Option<SaveForLaterTrackAttributes>,
    #[serde(default)]
    pub relationships: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveForLaterTrackAttributes {
    #[serde(default)]
    pub access_type: Option<String>,
    #[serde(default)]
    pub ai: Option<bool>,
    #[serde(default)]
    pub availability: Vec<String>,
    #[serde(default)]
    pub bpm: Option<f64>,
    #[serde(default)]
    pub copyright: Option<SaveForLaterItemsCopyright>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub duration: Option<String>,
    #[serde(default)]
    pub explicit: Option<bool>,
    #[serde(default)]
    pub external_links: Vec<SaveForLaterItemsExternalLink>,
    #[serde(default)]
    pub isrc: Option<String>,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub key_scale: Option<String>,
    #[serde(default)]
    pub media_tags: Vec<String>,
    #[serde(default)]
    pub popularity: Option<f64>,
    #[serde(default)]
    pub spotlighted: Option<bool>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub tone_tags: Vec<String>,
    #[serde(default)]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveForLaterAlbum {
    pub id: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    #[serde(default)]
    pub attributes: Option<SaveForLaterAlbumAttributes>,
    #[serde(default)]
    pub relationships: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveForLaterAlbumAttributes {
    #[serde(default)]
    pub access_type: Option<String>,
    #[serde(default)]
    pub ai: Option<bool>,
    #[serde(default)]
    pub album_type: Option<String>,
    #[serde(default)]
    pub availability: Vec<String>,
    #[serde(default)]
    pub barcode_id: Option<String>,
    #[serde(default)]
    pub copyright: Option<SaveForLaterItemsCopyright>,
    #[serde(default)]
    pub duration: Option<String>,
    #[serde(default)]
    pub explicit: Option<bool>,
    #[serde(default)]
    pub external_links: Vec<SaveForLaterItemsExternalLink>,
    #[serde(default)]
    pub media_tags: Vec<String>,
    #[serde(default)]
    pub number_of_items: Option<u32>,
    #[serde(default)]
    pub number_of_volumes: Option<u32>,
    #[serde(default)]
    pub popularity: Option<f64>,
    #[serde(default)]
    pub release_date: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveForLaterPlaylist {
    pub id: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    #[serde(default)]
    pub attributes: Option<SaveForLaterPlaylistAttributes>,
    #[serde(default)]
    pub relationships: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveForLaterPlaylistAttributes {
    #[serde(default)]
    pub access_type: Option<String>,
    #[serde(default)]
    pub bounded: Option<bool>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub duration: Option<String>,
    #[serde(default)]
    pub external_links: Vec<SaveForLaterItemsExternalLink>,
    #[serde(default)]
    pub last_modified_at: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub number_of_followers: Option<u32>,
    #[serde(default)]
    pub number_of_items: Option<u32>,
    #[serde(default)]
    pub number_of_track_items: Option<u32>,
    #[serde(default)]
    pub number_of_video_items: Option<u32>,
    #[serde(default)]
    pub playlist_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveForLaterArtwork {
    pub id: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    #[serde(default)]
    pub attributes: Option<SaveForLaterArtworkAttributes>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveForLaterArtworkAttributes {
    #[serde(default)]
    pub files: Vec<SaveForLaterItemsFile>,
    #[serde(default)]
    pub media_type: Option<String>,
    #[serde(default)]
    pub visual_metadata: Option<SaveForLaterItemsVisualMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveForLaterUnknownResource {
    pub id: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    #[serde(default)]
    pub attributes: Option<serde_json::Value>,
    #[serde(default)]
    pub relationships: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveForLaterItemsCopyright {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveForLaterItemsExternalLink {
    pub href: String,
    pub meta: SaveForLaterItemsExternalLinkMeta,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveForLaterItemsExternalLinkMeta {
    #[serde(default, rename = "type")]
    pub link_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveForLaterItemsFile {
    pub href: String,
    pub meta: SaveForLaterItemsFileMeta,
    #[serde(rename = "type")]
    pub file_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveForLaterItemsFileMeta {
    pub height: u32,
    pub width: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveForLaterItemsVisualMetadata {
    #[serde(default)]
    pub blur_hash: Option<String>,
    #[serde(default)]
    pub selected_palette_color: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{GetSaveForLaterResponse, SaveForLaterItemsIncludedResource};

    #[test]
    fn deserializes_resource_specific_included_attributes() {
        let response: GetSaveForLaterResponse = serde_json::from_value(serde_json::json!({
            "data": [],
            "included": [
                {
                    "id": "1",
                    "type": "tracks",
                    "attributes": {
                        "bpm": 151.0,
                        "mediaTags": ["LOSSLESS"],
                        "title": "Pill"
                    }
                },
                {
                    "id": "artwork",
                    "type": "artworks",
                    "attributes": {
                        "files": [{
                            "href": "https://example.com/image.jpg",
                            "meta": {"height": 1280, "width": 1280},
                            "type": "image/jpeg"
                        }]
                    }
                }
            ]
        }))
        .expect("save-for-later response should deserialize");

        let SaveForLaterItemsIncludedResource::Track(track) = &response.included[0] else {
            panic!("expected track resource");
        };
        assert_eq!(
            track.attributes.as_ref().unwrap().title.as_deref(),
            Some("Pill")
        );
        assert_eq!(track.attributes.as_ref().unwrap().bpm, Some(151.0));

        let SaveForLaterItemsIncludedResource::Artwork(artwork) = &response.included[1] else {
            panic!("expected artwork resource");
        };
        assert_eq!(
            artwork.attributes.as_ref().unwrap().files[0].meta.width,
            1280
        );
    }

    #[test]
    fn preserves_unknown_resource_types() {
        let response: GetSaveForLaterResponse = serde_json::from_value(serde_json::json!({
            "data": [],
            "included": [{
                "id": "1",
                "type": "new-resource",
                "attributes": {"futureField": true}
            }]
        }))
        .expect("unknown resource should deserialize");

        assert!(matches!(
            response.included[0],
            SaveForLaterItemsIncludedResource::Unknown(_)
        ));
    }
}
