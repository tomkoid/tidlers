use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeFeedPhone {
    pub uuid: String,
    pub page: Page,
    pub header: Header,
    pub items: Vec<HomeItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HomeFeedWeb {
    pub uuid: String,
    pub page: PageWeb,
    pub header: Header,
    pub items: Vec<HomeItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub cursor: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PageWeb {
    pub cursor: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub vibes: Vibes,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Vibes {
    pub items: Vec<serde_json::Value>, // Empty in example, using generic Value
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HomeItem {
    #[serde(rename_all = "camelCase")]
    ShortcutList {
        #[serde(flatten)]
        inner: Box<ShortcutList>,
    },
    #[serde(rename_all = "camelCase")]
    HorizontalList {
        #[serde(flatten)]
        inner: Box<HorizontalList>,
    },
    #[serde(rename_all = "camelCase")]
    HorizontalListWithContext {
        #[serde(flatten)]
        inner: Box<HorizontalListWithContext>,
    },
    #[serde(rename_all = "camelCase")]
    VerticalListCard {
        #[serde(flatten)]
        inner: Box<VerticalListCard>,
    },
    #[serde(rename_all = "camelCase")]
    TrackList {
        #[serde(flatten)]
        inner: Box<TrackList>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShortcutList {
    pub module_id: String,
    pub title: String,
    pub icons: Vec<String>,
    pub items: Vec<ShortcutItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ShortcutItem {
    Mix(ShortcutMix),
    DeepLink(ShortcutDeepLink),
    Playlist(ShortcutPlaylist),
    Album(ShortcutAlbum),
    Unknown(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShortcutDeepLink {
    #[serde(rename = "type")]
    pub item_type: String, // "DEEP_LINK"
    pub following: bool,
    pub data: DeepLinkData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShortcutPlaylist {
    #[serde(rename = "type")]
    pub item_type: String, // "PLAYLIST"
    pub following: bool,
    #[serde(default)]
    pub number_of_followers: Option<u32>,
    pub data: PlaylistData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShortcutAlbum {
    #[serde(rename = "type")]
    pub item_type: String, // "ALBUM"
    pub following: bool,
    pub data: AlbumData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShortcutMix {
    #[serde(rename = "type")]
    pub item_type: String, // "MIX"
    pub following: bool,
    pub data: MixData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeepLinkData {
    pub title: String,
    pub id: String,
    pub url: String,
    pub external_url: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalList {
    pub module_id: String,
    pub title: String,
    pub icons: Vec<String>,
    pub show_quick_play: bool,
    pub view_all: Option<String>,
    pub items: Vec<ListItem>,
    #[serde(default)]
    pub subtitle: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HorizontalListWithContext {
    pub module_id: String,
    pub title: String,
    pub icons: Vec<String>,
    pub show_quick_play: bool,
    pub view_all: Option<String>,
    pub header: ListItem,
    pub items: Vec<ListItem>,
    #[serde(default)]
    pub subtitle: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VerticalListCard {
    pub module_id: String,
    pub title: String,
    pub icons: Vec<String>,
    pub items: Vec<ListItem>,
    #[serde(default)]
    pub subtitle: Option<String>,
    pub view_all: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TrackList {
    pub module_id: String,
    pub title: String,
    pub icons: Vec<String>,
    pub items: Vec<ListItem>,
    #[serde(default)]
    pub subtitle: Option<String>,
    #[serde(default)]
    pub view_all: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ListItem {
    Mix(ListMix),
    Artist(ListArtist),
    Playlist(ListPlaylist),
    Album(ListAlbum),
    Track(ListTrack),
    DeepLink(ListDeepLink),
    Unknown(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListArtist {
    #[serde(rename = "type")]
    pub item_type: String,
    pub following: bool,
    pub data: ArtistData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListMix {
    #[serde(rename = "type")]
    pub item_type: String,
    pub following: bool,
    pub data: MixData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListPlaylist {
    #[serde(rename = "type")]
    pub item_type: String,
    pub following: bool,
    #[serde(default)]
    pub number_of_followers: Option<u32>,
    pub data: PlaylistData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListAlbum {
    #[serde(rename = "type")]
    pub item_type: String,
    pub following: bool,
    pub data: AlbumData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListTrack {
    #[serde(rename = "type")]
    pub item_type: String,
    pub following: bool,
    pub data: TrackData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListDeepLink {
    #[serde(rename = "type")]
    pub item_type: String,
    pub following: bool,
    pub data: DeepLinkData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArtistData {
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
pub struct ArtistRole {
    pub category_id: i32,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MixData {
    #[serde(rename = "type")]
    pub mix_type: String,
    pub country_code: String,
    pub updated: u64,
    pub id: String,
    pub is_stable_id: bool,
    pub artifact_id_type: String,
    pub mix_images: Vec<ImageInfo>,
    pub sort_type: String,
    pub detail_mix_images: Vec<ImageInfo>,
    pub title_text_info: TextInfo,
    pub subtitle_text_info: TextInfo,
    pub short_subtitle_text_info: TextInfo,
    pub description: TextInfo,
    pub content_behavior: String,
    #[serde(default)]
    pub user_id: Option<u64>,
    #[serde(default)]
    pub artists: Option<Vec<MixArtist>>,
    #[serde(default)]
    pub user_top_artists: Option<Vec<MixArtist>>,
    #[serde(default)]
    pub mix_date: Option<String>,
    #[serde(default)]
    pub sharing_images: Option<HashMap<String, Vec<ImageInfo>>>,
    #[serde(default)]
    pub at_date: Option<String>,
    #[serde(default)]
    pub mix_number: Option<u32>,
    #[serde(default)]
    pub track: Option<MixTrack>,
    #[serde(default)]
    pub artist: Option<MixArtistInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImageInfo {
    pub size: String,
    pub url: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TextInfo {
    pub text: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MixArtist {
    pub artist_id: u64,
    pub artist_name: String,
    pub artist_image: ArtistImage,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArtistImage {
    pub image_uuid: String,
    pub vibrant_color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MixTrack {
    pub track_id: u64,
    pub track_group: String,
    pub track_title: String,
    pub track_image: ArtistImage,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MixArtistInfo {
    pub artist_id: u64,
    pub artist_name: String,
    pub artist_image: ArtistImage,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistData {
    pub uuid: String,
    #[serde(rename = "type")]
    pub playlist_type: String,
    pub creator: Creator,
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
    pub promoted_artists: Vec<PromotedArtist>,
    pub trn: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
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
pub struct PromotedArtist {
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub artist_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AlbumData {
    pub id: u64,
    pub title: String,
    pub artists: Vec<AlbumArtist>,
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
    pub master_album: Option<MasterAlbum>,
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
pub struct AlbumArtist {
    pub id: u64,
    pub name: String,
    pub handle: Option<String>,
    pub picture: Option<String>,
    pub main: bool,
    pub user_id: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MasterAlbum {
    pub id: String,
    pub release_date: String,
    pub categories: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MediaMetadata {
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TrackData {
    pub id: u64,
    pub editable: bool,
    pub title: String,
    pub album: TrackAlbum,
    pub artists: Vec<AlbumArtist>,
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
    pub genres: Vec<Genre>,
    pub audio_analysis_attributes: Option<AudioAnalysisAttributes>,
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
pub struct TrackAlbum {
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
pub struct Genre {
    pub id: u32,
    pub name: String,
    pub parent_id: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AudioAnalysisAttributes {
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
