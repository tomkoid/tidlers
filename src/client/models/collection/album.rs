use serde::{Deserialize, Serialize};

use crate::client::models::album::Album;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionFavoriteAlbumsResponse {
    pub items: Vec<CollectionFavoriteAlbumEntry>,
    pub limit: i32,
    pub offset: i32,
    pub total_number_of_items: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionFavoriteAlbumEntry {
    pub created: String,
    pub item: Album,
}

#[cfg(test)]
mod tests {
    use super::CollectionFavoriteAlbumsResponse;
    use crate::client::models::album::Album;
    use serde_json::{Value, json};

    fn favorite(album: Value) -> Album {
        let response: CollectionFavoriteAlbumsResponse = serde_json::from_value(json!({
            "limit": 100, "offset": 0, "totalNumberOfItems": 1,
            "items": [{"created": "2026-01-01T00:00:00Z", "item": album}]
        }))
        .expect("favorite-albums response should parse");
        response.items.into_iter().next().unwrap().item
    }

    #[test]
    fn retains_catalogue_metadata_from_favorite_album_entries() {
        let album = favorite(json!({
            "id": 123, "title": "Example album", "cover": "cover-id",
            "releaseDate": "2026-01-01", "artist": {"id": 7, "name": "Example artist"},
            "numberOfTracks": 12, "duration": 3600, "explicit": true,
            "audioQuality": "LOSSLESS", "mediaMetadata": {"tags": ["LOSSLESS", "HIRES_LOSSLESS"]}
        }));
        let artist = album.artist.as_ref().unwrap();
        assert_eq!(artist.id, 7);
        assert_eq!(artist.name, "Example artist");
        assert_eq!(album.number_of_tracks, Some(12));
        assert_eq!(album.duration, Some(3600));
        assert_eq!(album.explicit, Some(true));
        assert_eq!(album.audio_quality.as_deref(), Some("LOSSLESS"));
        assert_eq!(
            album.media_metadata.as_ref().unwrap().tags,
            ["LOSSLESS", "HIRES_LOSSLESS"]
        );
        let value = serde_json::to_value(&album).unwrap();
        assert_eq!(value["numberOfTracks"], 12);
        assert_eq!(value["mediaMetadata"]["tags"][1], "HIRES_LOSSLESS");
    }

    #[test]
    fn minimal_embedded_album_remains_valid() {
        let album: Album =
            serde_json::from_value(json!({"id": 123, "title": "Album", "cover": "cover-id"}))
                .unwrap();
        assert_eq!(album.id, 123);
        assert_eq!(album.cover.as_deref(), Some("cover-id"));
        assert!(album.artist.is_none());
        assert!(album.number_of_tracks.is_none());
        assert!(album.duration.is_none());
        assert!(album.explicit.is_none());
        assert!(album.audio_quality.is_none());
        assert!(album.media_metadata.is_none());
        assert!(
            serde_json::to_value(&album)
                .unwrap()
                .get("artist")
                .is_none()
        );
    }

    #[test]
    fn partial_collection_metadata_does_not_require_other_fields() {
        let album =
            favorite(json!({"id": 123, "title": "Album", "artist": {"id": 7, "name": "Artist"}}));
        assert_eq!(album.artist.unwrap().name, "Artist");
        assert!(album.number_of_tracks.is_none());
        assert!(album.duration.is_none());
        assert!(album.explicit.is_none());
        assert!(album.audio_quality.is_none());
        assert!(album.media_metadata.is_none());
    }

    #[test]
    fn null_enrichment_is_treated_as_absent() {
        let album = favorite(json!({"id": 123, "title": "Album", "artist": null,
            "numberOfTracks": null, "duration": null, "explicit": null,
            "audioQuality": null, "mediaMetadata": null}));
        assert!(album.artist.is_none());
        assert!(album.number_of_tracks.is_none());
        assert!(album.duration.is_none());
        assert!(album.explicit.is_none());
        assert!(album.audio_quality.is_none());
        assert!(album.media_metadata.is_none());
    }

    #[test]
    fn preserves_explicit_zero_false_and_empty_tags() {
        let album = favorite(json!({"id": 123, "title": "Album", "numberOfTracks": 0,
            "duration": 0, "explicit": false, "mediaMetadata": {"tags": []}}));
        assert_eq!(album.number_of_tracks, Some(0));
        assert_eq!(album.duration, Some(0));
        assert_eq!(album.explicit, Some(false));
        assert!(album.media_metadata.unwrap().tags.is_empty());
    }
}
