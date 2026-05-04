use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::client::models::{artist::ArtistRole, media::MediaMetadata};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeFeedPhone {
    pub uuid: String,
    pub page: HomePageCursor,
    pub header: HomeHeader,
    pub items: Vec<HomeItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeFeedWeb {
    pub uuid: String,
    pub page: HomePageCursorWeb,
    pub header: HomeHeader,
    pub items: Vec<HomeItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomePageCursor {
    pub cursor: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomePageCursorWeb {
    pub cursor: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeHeader {
    pub vibes: HomeVibes,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeVibes {
    pub items: Vec<serde_json::Value>, // Empty in example, using generic Value
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HomeItem {
    #[serde(rename_all = "camelCase")]
    HomeShortcutList {
        #[serde(flatten)]
        inner: Box<HomeShortcutList>,
    },
    #[serde(rename_all = "camelCase")]
    HomeHorizontalList {
        #[serde(flatten)]
        inner: Box<HomeHorizontalList>,
    },
    #[serde(rename_all = "camelCase")]
    HomeHorizontalListWithContext {
        #[serde(flatten)]
        inner: Box<HomeHorizontalListWithContext>,
    },
    #[serde(rename_all = "camelCase")]
    HomeVerticalListCard {
        #[serde(flatten)]
        inner: Box<HomeVerticalListCard>,
    },
    #[serde(rename_all = "camelCase")]
    HomeTrackList {
        #[serde(flatten)]
        inner: Box<HomeTrackList>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeShortcutList {
    pub module_id: String,
    pub title: String,
    pub icons: Vec<String>,
    pub items: Vec<HomeShortcutItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum HomeShortcutItem {
    Mix(HomeShortcutMix),
    DeepLink(HomeShortcutDeepLink),
    Playlist(HomeShortcutPlaylist),
    Album(HomeShortcutAlbum),
    Unknown(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeShortcutDeepLink {
    #[serde(rename = "type")]
    pub item_type: String, // "DEEP_LINK"
    pub following: bool,
    pub data: HomeDeepLinkData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeShortcutPlaylist {
    #[serde(rename = "type")]
    pub item_type: String, // "PLAYLIST"
    pub following: bool,
    #[serde(default)]
    pub number_of_followers: Option<u32>,
    pub data: HomePlaylistData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeShortcutAlbum {
    #[serde(rename = "type")]
    pub item_type: String, // "ALBUM"
    pub following: bool,
    pub data: HomeAlbumData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeShortcutMix {
    #[serde(rename = "type")]
    pub item_type: String, // "MIX"
    pub following: bool,
    pub data: HomeMixData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeDeepLinkData {
    pub title: String,
    pub id: String,
    pub url: String,
    pub external_url: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeHorizontalList {
    pub module_id: String,
    pub title: String,
    pub icons: Vec<String>,
    pub show_quick_play: bool,
    pub view_all: Option<String>,
    pub items: Vec<HomeListItem>,
    #[serde(default)]
    pub subtitle: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeHorizontalListWithContext {
    pub module_id: String,
    pub title: String,
    pub icons: Vec<String>,
    pub show_quick_play: bool,
    pub view_all: Option<String>,
    pub header: HomeListItem,
    pub items: Vec<HomeListItem>,
    #[serde(default)]
    pub subtitle: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeVerticalListCard {
    pub module_id: String,
    pub title: String,
    pub icons: Vec<String>,
    pub items: Vec<HomeListItem>,
    #[serde(default)]
    pub subtitle: Option<String>,
    pub view_all: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeTrackList {
    pub module_id: String,
    pub title: String,
    pub icons: Vec<String>,
    pub items: Vec<HomeListItem>,
    #[serde(default)]
    pub subtitle: Option<String>,
    #[serde(default)]
    pub view_all: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum HomeListItem {
    Mix(HomeListMix),
    Artist(HomeListArtist),
    Playlist(HomeListPlaylist),
    Album(HomeListAlbum),
    Track(HomeListTrack),
    DeepLink(HomeListDeepLink),
    Unknown(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeListArtist {
    #[serde(rename = "type")]
    pub item_type: String,
    pub following: bool,
    pub data: HomeArtistData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeListMix {
    #[serde(rename = "type")]
    pub item_type: String,
    pub following: bool,
    pub data: HomeMixData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeListPlaylist {
    #[serde(rename = "type")]
    pub item_type: String,
    pub following: bool,
    #[serde(default)]
    pub number_of_followers: Option<u32>,
    pub data: HomePlaylistData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeListAlbum {
    #[serde(rename = "type")]
    pub item_type: String,
    pub following: bool,
    pub data: HomeAlbumData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeListTrack {
    #[serde(rename = "type")]
    pub item_type: String,
    pub following: bool,
    pub data: HomeTrackData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeListDeepLink {
    #[serde(rename = "type")]
    pub item_type: String,
    pub following: bool,
    pub data: HomeDeepLinkData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeArtistData {
    pub id: u64,
    pub name: String,
    pub picture: Option<String>,
    pub popularity: u32,
    pub double_popularity: f64,
    pub artist_types: Vec<String>,
    pub artist_roles: Vec<ArtistRole>,
    pub mixes: HashMap<String, String>,
    pub vibrant_color: Option<String>,
    pub selected_album_cover_fallback: Option<String>,
    pub handle: Option<String>,
    pub user_id: Option<u64>,
    pub artwork_id: Option<String>,
    pub spotlighted: bool,
    pub contributions_enabled: bool,
    pub cash_app_onboarded: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeMixData {
    #[serde(rename = "type")]
    pub mix_type: String,
    pub country_code: String,
    pub updated: u64,
    pub id: String,
    pub is_stable_id: bool,
    pub artifact_id_type: String,
    pub mix_images: Vec<HomeImageInfo>,
    pub sort_type: String,
    pub detail_mix_images: Vec<HomeImageInfo>,
    pub title_text_info: HomeTextInfo,
    pub subtitle_text_info: HomeTextInfo,
    pub short_subtitle_text_info: HomeTextInfo,
    pub description: HomeTextInfo,
    pub content_behavior: String,
    #[serde(default)]
    pub user_id: Option<u64>,
    #[serde(default)]
    pub artists: Option<Vec<HomeMixArtist>>,
    #[serde(default)]
    pub user_top_artists: Option<Vec<HomeMixArtist>>,
    #[serde(default)]
    pub mix_date: Option<String>,
    #[serde(default)]
    pub sharing_images: Option<HashMap<String, Vec<HomeImageInfo>>>,
    #[serde(default)]
    pub at_date: Option<String>,
    #[serde(default)]
    pub mix_number: Option<u32>,
    #[serde(default)]
    pub track: Option<HomeMixTrack>,
    #[serde(default)]
    pub artist: Option<HomeMixArtistInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeImageInfo {
    pub size: String,
    pub url: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeTextInfo {
    pub text: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeMixArtist {
    pub artist_id: u64,
    pub artist_name: String,
    pub artist_image: HomeArtistImage,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeArtistImage {
    pub image_uuid: String,
    pub vibrant_color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeMixTrack {
    pub track_id: u64,
    pub track_group: String,
    pub track_title: String,
    pub track_image: HomeArtistImage,
}

pub type HomeMixArtistInfo = HomeMixArtist;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomePlaylistData {
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
    pub image: Option<String>,
    pub square_image: Option<String>,
    pub artwork_id: Option<String>,
    pub url: String,
    pub created: String,
    pub last_updated: String,
    pub last_item_added_at: String,
    pub duration: u32,
    pub number_of_tracks: u32,
    pub number_of_videos: u32,
    pub promoted_artists: Vec<HomePromotedArtist>,
    pub trn: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CollectionCreator {
    pub id: u64,
    pub name: Option<String>,
    pub picture: Option<String>,
    #[serde(rename = "type")]
    pub creator_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Curator {
    pub id: u64,
    pub name: String,
    pub handle: Option<String>,
    pub picture: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomePromotedArtist {
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub artist_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeAlbumData {
    pub id: u64,
    pub title: String,
    pub artists: Vec<HomeAlbumArtist>,
    pub duration: u32,
    pub cover: String,
    pub vibrant_color: Option<String>,
    pub video_cover: Option<String>,
    pub copyright: String,
    pub number_of_volumes: u32,
    pub number_of_tracks: u32,
    pub number_of_videos: u32,
    pub popularity: u32,
    pub double_popularity: f64,
    pub version: Option<String>,
    pub release_date: String,
    #[serde(rename = "type")]
    pub album_type: String,
    pub explicit: bool,
    pub upc: String,
    pub audio_quality: String,
    pub master_album: Option<HomeMasterAlbum>,
    pub allow_streaming: bool,
    pub stream_start_date: String,
    pub stream_ready: bool,
    pub pay_to_stream: bool,
    pub audio_modes: Vec<String>,
    pub ad_supported_stream_ready: bool,
    pub media_metadata: MediaMetadata,
    pub provider_id: u32,
    pub provider_name: String,
    pub dj_ready: bool,
    pub stem_ready: bool,
    pub upload: bool,
    pub access_type: String,
    pub created_at: Option<String>,
    pub user_id: Option<u64>,
    pub artwork_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeAlbumArtist {
    pub id: u64,
    pub name: String,
    pub handle: Option<String>,
    pub picture: Option<String>,
    pub main: bool,
    pub user_id: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeMasterAlbum {
    pub id: String,
    pub release_date: String,
    pub categories: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeTrackData {
    pub id: u64,
    pub editable: bool,
    pub title: String,
    pub album: HomeTrackAlbum,
    pub artists: Vec<HomeAlbumArtist>,
    pub version: Option<String>,
    pub duration: u32,
    pub popularity: u32,
    pub double_popularity: f64,
    pub track_number: u32,
    pub volume_number: u32,
    pub explicit: bool,
    pub replay_gain: f64,
    pub audio_quality: String,
    pub allow_streaming: bool,
    pub stream_start_date: String,
    pub stream_ready: bool,
    pub audio_modes: Vec<String>,
    pub mixes: HashMap<String, String>,
    pub ad_supported_stream_ready: bool,
    pub media_metadata: MediaMetadata,
    pub provider_id: u32,
    pub provider_name: String,
    pub dj_ready: bool,
    pub stem_ready: bool,
    pub pay_to_stream: bool,
    pub genres: Vec<HomeGenre>,
    pub audio_analysis_attributes: Option<HomeAudioAnalysisAttributes>,
    pub upload: bool,
    pub access_type: String,
    pub spotlighted: bool,
    pub created_at: Option<String>,
    pub user_id: Option<u64>,
    pub isrc: String,
    pub peak: f64,
    pub pre_paywall_presentation: String,
    pub copyright: String,
    pub track_group: String,
    pub first_available: String,
    pub linked_stereo_isrc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeTrackAlbum {
    pub id: u64,
    pub title: String,
    pub version: Option<String>,
    pub cover: String,
    pub vibrant_color: Option<String>,
    pub video_cover: Option<String>,
    pub release_date: String,
    pub upload: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeGenre {
    pub id: u32,
    pub name: String,
    pub parent_id: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeAudioAnalysisAttributes {
    #[serde(default)]
    pub scale: Option<String>,
    #[serde(default)]
    pub bpm: Option<String>,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub key_scale: Option<String>,
    #[serde(default)]
    pub tone_tags: Option<String>,
}
