use crate::{client::TidalClient, error::TidalError, urls::OPEN_API_V2_LOCATION};
use serde_json::Value;

macro_rules! openapi_get_no_id_methods {
    ($(($name:ident, $path:literal)),+ $(,)?) => {
        $(pub async fn $name(&self, query_params: &[(&str, &str)]) -> Result<Value, TidalError> {
            self.openapi_get_json($path, query_params).await
        })+
    };
}

macro_rules! openapi_get_id_methods {
    ($(($name:ident, $path:literal)),+ $(,)?) => {
        $(pub async fn $name(
            &self,
            id: impl Into<String>,
            query_params: &[(&str, &str)],
        ) -> Result<Value, TidalError> {
            let id = id.into();
            self.openapi_get_json($path.replace("{id}", &id), query_params).await
        })+
    };
}

impl TidalClient {
    async fn openapi_get_json(
        &self,
        path: impl Into<String>,
        query_params: &[(&str, &str)],
    ) -> Result<Value, TidalError> {
        let mut request = self
            .request(reqwest::Method::GET, path.into())
            .with_base_url(OPEN_API_V2_LOCATION);

        for (key, value) in query_params {
            request = request.with_param(*key, *value);
        }

        request.send().await
    }

    /// Generic helper for querying any OpenAPI v2 GET endpoint path directly.
    pub async fn openapi_get_path(
        &self,
        path: impl Into<String>,
        query_params: &[(&str, &str)],
    ) -> Result<Value, TidalError> {
        self.openapi_get_json(path, query_params).await
    }

    openapi_get_no_id_methods!(
        (openapi_get_albums, "/albums"),
        (openapi_get_artists, "/artists"),
        (openapi_get_credits, "/credits"),
        (openapi_get_lyrics, "/lyrics"),
        (openapi_get_playlists, "/playlists"),
        (openapi_get_tracks, "/tracks"),
        (
            openapi_get_user_collection_folders,
            "/userCollectionFolders"
        ),
        (openapi_get_videos, "/videos"),
    );

    openapi_get_id_methods!(
        (openapi_get_albums_by_id, "/albums/{id}"),
        (
            openapi_get_albums_by_id_relationships_album_statistics,
            "/albums/{id}/relationships/albumStatistics"
        ),
        (
            openapi_get_albums_by_id_relationships_artists,
            "/albums/{id}/relationships/artists"
        ),
        (
            openapi_get_albums_by_id_relationships_cover_art,
            "/albums/{id}/relationships/coverArt"
        ),
        (
            openapi_get_albums_by_id_relationships_genres,
            "/albums/{id}/relationships/genres"
        ),
        (
            openapi_get_albums_by_id_relationships_items,
            "/albums/{id}/relationships/items"
        ),
        (
            openapi_get_albums_by_id_relationships_owners,
            "/albums/{id}/relationships/owners"
        ),
        (
            openapi_get_albums_by_id_relationships_price_config,
            "/albums/{id}/relationships/priceConfig"
        ),
        (
            openapi_get_albums_by_id_relationships_providers,
            "/albums/{id}/relationships/providers"
        ),
        (
            openapi_get_albums_by_id_relationships_similar_albums,
            "/albums/{id}/relationships/similarAlbums"
        ),
        (
            openapi_get_albums_by_id_relationships_suggested_cover_arts,
            "/albums/{id}/relationships/suggestedCoverArts"
        ),
        (
            openapi_get_albums_by_id_relationships_usage_rules,
            "/albums/{id}/relationships/usageRules"
        ),
        (openapi_get_artists_by_id, "/artists/{id}"),
        (
            openapi_get_artists_by_id_relationships_albums,
            "/artists/{id}/relationships/albums"
        ),
        (
            openapi_get_artists_by_id_relationships_biography,
            "/artists/{id}/relationships/biography"
        ),
        (
            openapi_get_artists_by_id_relationships_followers,
            "/artists/{id}/relationships/followers"
        ),
        (
            openapi_get_artists_by_id_relationships_following,
            "/artists/{id}/relationships/following"
        ),
        (
            openapi_get_artists_by_id_relationships_owners,
            "/artists/{id}/relationships/owners"
        ),
        (
            openapi_get_artists_by_id_relationships_profile_art,
            "/artists/{id}/relationships/profileArt"
        ),
        (
            openapi_get_artists_by_id_relationships_radio,
            "/artists/{id}/relationships/radio"
        ),
        (
            openapi_get_artists_by_id_relationships_roles,
            "/artists/{id}/relationships/roles"
        ),
        (
            openapi_get_artists_by_id_relationships_similar_artists,
            "/artists/{id}/relationships/similarArtists"
        ),
        (
            openapi_get_artists_by_id_relationships_track_providers,
            "/artists/{id}/relationships/trackProviders"
        ),
        (
            openapi_get_artists_by_id_relationships_tracks,
            "/artists/{id}/relationships/tracks"
        ),
        (
            openapi_get_artists_by_id_relationships_videos,
            "/artists/{id}/relationships/videos"
        ),
        (openapi_get_credits_by_id, "/credits/{id}"),
        (
            openapi_get_credits_by_id_relationships_artist,
            "/credits/{id}/relationships/artist"
        ),
        (
            openapi_get_credits_by_id_relationships_category,
            "/credits/{id}/relationships/category"
        ),
        (openapi_get_lyrics_by_id, "/lyrics/{id}"),
        (
            openapi_get_lyrics_by_id_relationships_owners,
            "/lyrics/{id}/relationships/owners"
        ),
        (
            openapi_get_lyrics_by_id_relationships_track,
            "/lyrics/{id}/relationships/track"
        ),
        (openapi_get_playlists_by_id, "/playlists/{id}"),
        (
            openapi_get_playlists_by_id_relationships_cover_art,
            "/playlists/{id}/relationships/coverArt"
        ),
        (
            openapi_get_playlists_by_id_relationships_items,
            "/playlists/{id}/relationships/items"
        ),
        (
            openapi_get_playlists_by_id_relationships_owner_profiles,
            "/playlists/{id}/relationships/ownerProfiles"
        ),
        (
            openapi_get_playlists_by_id_relationships_owners,
            "/playlists/{id}/relationships/owners"
        ),
        (openapi_get_search_results_by_id, "/searchResults/{id}"),
        (
            openapi_get_search_results_by_id_relationships_albums,
            "/searchResults/{id}/relationships/albums"
        ),
        (
            openapi_get_search_results_by_id_relationships_artists,
            "/searchResults/{id}/relationships/artists"
        ),
        (
            openapi_get_search_results_by_id_relationships_playlists,
            "/searchResults/{id}/relationships/playlists"
        ),
        (
            openapi_get_search_results_by_id_relationships_top_hits,
            "/searchResults/{id}/relationships/topHits"
        ),
        (
            openapi_get_search_results_by_id_relationships_tracks,
            "/searchResults/{id}/relationships/tracks"
        ),
        (
            openapi_get_search_results_by_id_relationships_videos,
            "/searchResults/{id}/relationships/videos"
        ),
        (openapi_get_tracks_by_id, "/tracks/{id}"),
        (
            openapi_get_tracks_by_id_relationships_albums,
            "/tracks/{id}/relationships/albums"
        ),
        (
            openapi_get_tracks_by_id_relationships_artists,
            "/tracks/{id}/relationships/artists"
        ),
        (
            openapi_get_tracks_by_id_relationships_credits,
            "/tracks/{id}/relationships/credits"
        ),
        (
            openapi_get_tracks_by_id_relationships_download,
            "/tracks/{id}/relationships/download"
        ),
        (
            openapi_get_tracks_by_id_relationships_genres,
            "/tracks/{id}/relationships/genres"
        ),
        (
            openapi_get_tracks_by_id_relationships_lyrics,
            "/tracks/{id}/relationships/lyrics"
        ),
        (
            openapi_get_tracks_by_id_relationships_metadata_status,
            "/tracks/{id}/relationships/metadataStatus"
        ),
        (
            openapi_get_tracks_by_id_relationships_owners,
            "/tracks/{id}/relationships/owners"
        ),
        (
            openapi_get_tracks_by_id_relationships_price_config,
            "/tracks/{id}/relationships/priceConfig"
        ),
        (
            openapi_get_tracks_by_id_relationships_providers,
            "/tracks/{id}/relationships/providers"
        ),
        (
            openapi_get_tracks_by_id_relationships_radio,
            "/tracks/{id}/relationships/radio"
        ),
        (
            openapi_get_tracks_by_id_relationships_replacement,
            "/tracks/{id}/relationships/replacement"
        ),
        (
            openapi_get_tracks_by_id_relationships_shares,
            "/tracks/{id}/relationships/shares"
        ),
        (
            openapi_get_tracks_by_id_relationships_similar_tracks,
            "/tracks/{id}/relationships/similarTracks"
        ),
        (
            openapi_get_tracks_by_id_relationships_source_file,
            "/tracks/{id}/relationships/sourceFile"
        ),
        (
            openapi_get_tracks_by_id_relationships_suggested_tracks,
            "/tracks/{id}/relationships/suggestedTracks"
        ),
        (
            openapi_get_tracks_by_id_relationships_track_statistics,
            "/tracks/{id}/relationships/trackStatistics"
        ),
        (
            openapi_get_tracks_by_id_relationships_usage_rules,
            "/tracks/{id}/relationships/usageRules"
        ),
        (
            openapi_get_user_collection_albums_by_id,
            "/userCollectionAlbums/{id}"
        ),
        (
            openapi_get_user_collection_albums_by_id_relationships_items,
            "/userCollectionAlbums/{id}/relationships/items"
        ),
        (
            openapi_get_user_collection_albums_by_id_relationships_owners,
            "/userCollectionAlbums/{id}/relationships/owners"
        ),
        (
            openapi_get_user_collection_artists_by_id,
            "/userCollectionArtists/{id}"
        ),
        (
            openapi_get_user_collection_artists_by_id_relationships_items,
            "/userCollectionArtists/{id}/relationships/items"
        ),
        (
            openapi_get_user_collection_artists_by_id_relationships_owners,
            "/userCollectionArtists/{id}/relationships/owners"
        ),
        (
            openapi_get_user_collection_folders_by_id,
            "/userCollectionFolders/{id}"
        ),
        (
            openapi_get_user_collection_folders_by_id_relationships_items,
            "/userCollectionFolders/{id}/relationships/items"
        ),
        (
            openapi_get_user_collection_folders_by_id_relationships_owners,
            "/userCollectionFolders/{id}/relationships/owners"
        ),
        (
            openapi_get_user_collection_playlists_by_id,
            "/userCollectionPlaylists/{id}"
        ),
        (
            openapi_get_user_collection_playlists_by_id_relationships_items,
            "/userCollectionPlaylists/{id}/relationships/items"
        ),
        (
            openapi_get_user_collection_playlists_by_id_relationships_owners,
            "/userCollectionPlaylists/{id}/relationships/owners"
        ),
        (
            openapi_get_user_collection_tracks_by_id,
            "/userCollectionTracks/{id}"
        ),
        (
            openapi_get_user_collection_tracks_by_id_relationships_items,
            "/userCollectionTracks/{id}/relationships/items"
        ),
        (
            openapi_get_user_collection_tracks_by_id_relationships_owners,
            "/userCollectionTracks/{id}/relationships/owners"
        ),
        (
            openapi_get_user_collection_videos_by_id,
            "/userCollectionVideos/{id}"
        ),
        (
            openapi_get_user_collection_videos_by_id_relationships_items,
            "/userCollectionVideos/{id}/relationships/items"
        ),
        (
            openapi_get_user_collection_videos_by_id_relationships_owners,
            "/userCollectionVideos/{id}/relationships/owners"
        ),
        (openapi_get_user_collections_by_id, "/userCollections/{id}"),
        (
            openapi_get_user_collections_by_id_relationships_albums,
            "/userCollections/{id}/relationships/albums"
        ),
        (
            openapi_get_user_collections_by_id_relationships_artists,
            "/userCollections/{id}/relationships/artists"
        ),
        (
            openapi_get_user_collections_by_id_relationships_owners,
            "/userCollections/{id}/relationships/owners"
        ),
        (
            openapi_get_user_collections_by_id_relationships_playlists,
            "/userCollections/{id}/relationships/playlists"
        ),
        (
            openapi_get_user_collections_by_id_relationships_tracks,
            "/userCollections/{id}/relationships/tracks"
        ),
        (
            openapi_get_user_collections_by_id_relationships_videos,
            "/userCollections/{id}/relationships/videos"
        ),
        (openapi_get_videos_by_id, "/videos/{id}"),
        (
            openapi_get_videos_by_id_relationships_albums,
            "/videos/{id}/relationships/albums"
        ),
        (
            openapi_get_videos_by_id_relationships_artists,
            "/videos/{id}/relationships/artists"
        ),
        (
            openapi_get_videos_by_id_relationships_credits,
            "/videos/{id}/relationships/credits"
        ),
        (
            openapi_get_videos_by_id_relationships_providers,
            "/videos/{id}/relationships/providers"
        ),
        (
            openapi_get_videos_by_id_relationships_replacement,
            "/videos/{id}/relationships/replacement"
        ),
        (
            openapi_get_videos_by_id_relationships_similar_videos,
            "/videos/{id}/relationships/similarVideos"
        ),
        (
            openapi_get_videos_by_id_relationships_suggested_videos,
            "/videos/{id}/relationships/suggestedVideos"
        ),
        (
            openapi_get_videos_by_id_relationships_thumbnail_art,
            "/videos/{id}/relationships/thumbnailArt"
        ),
        (
            openapi_get_videos_by_id_relationships_usage_rules,
            "/videos/{id}/relationships/usageRules"
        ),
    );
}
