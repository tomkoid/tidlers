use crate::client::models::{artist::ArtistRole, media::MediaMetadata};
use serde::{Deserialize, Deserializer, Serialize, de};
use std::collections::HashMap;

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
    #[serde(rename = "SHORTCUT_LIST", rename_all = "camelCase")]
    HomeShortcutList {
        #[serde(flatten)]
        inner: Box<HomeShortcutList>,
    },
    #[serde(rename = "HORIZONTAL_LIST", rename_all = "camelCase")]
    HomeHorizontalList {
        #[serde(flatten)]
        inner: Box<HomeHorizontalList>,
    },
    #[serde(rename = "HORIZONTAL_LIST_WITH_CONTEXT", rename_all = "camelCase")]
    HomeHorizontalListWithContext {
        #[serde(flatten)]
        inner: Box<HomeHorizontalListWithContext>,
    },
    #[serde(rename = "VERTICAL_LIST_CARD", rename_all = "camelCase")]
    HomeVerticalListCard {
        #[serde(flatten)]
        inner: Box<HomeVerticalListCard>,
    },
    #[serde(rename = "TRACK_LIST", rename_all = "camelCase")]
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
    #[serde(deserialize_with = "mix_data_tolerating_repeated_keys")]
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
    #[serde(deserialize_with = "mix_data_tolerating_repeated_keys")]
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

/// TIDAL sends every mix's `type` twice inside its `data` object, with the same
/// value. A derived struct rejects a repeated field, and the untagged item
/// enums would then quietly turn every mix into `Unknown`. Decoding through a
/// [`serde_json::Value`] keeps one copy of the key before the strict struct
/// sees it.
fn mix_data_tolerating_repeated_keys<'de, D>(deserializer: D) -> Result<HomeMixData, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    serde_json::from_value(value).map_err(de::Error::custom)
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
    /// Null for some mixes (seen on track mixes).
    pub artist_image: Option<HomeArtistImage>,
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
    /// Null for some tracks, as on [`Track`](crate::client::models::track::Track).
    pub mixes: Option<HashMap<String, String>>,
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

#[cfg(test)]
mod tests {
    use super::{HomeFeedWeb, HomeItem, HomeListItem, HomeShortcutItem};

    /// Trimmed from a live `/v2/home/feed/static` response (deviceType=BROWSER):
    /// a shortcut mix, a list with a daily mix and a track mix, and a track
    /// list. Kept as raw text on purpose: every mix's `data` repeats its `type`
    /// key, which a `serde_json::Value` round trip would drop. User and mix ids
    /// are placeholders.
    const FEED: &str = r##"{"uuid":"00000000-0000-0000-0000-000000000000","page":{"cursor":"SElTVE9SWV9NSVhFUw"},"header":{"vibes":{"items":[{"name":"For you","type":"STATIC"},{"name":"Staff Picks","type":"EDITORIAL"},{"name":"Uploads","type":"UPLOADS"}]}},"items":[{"type":"SHORTCUT_LIST","moduleId":"SHORTCUT_LIST","title":"Shortcuts","icons":[],"items":[{"type":"MIX","following":true,"data":{"type":"DISCOVERY_MIX","countryCode":"WW","updated":1791880510041,"id":"000000000000000000000000000002","isStableId":true,"artifactIdType":"trackGroupId","mixImages":[{"size":"SMALL","url":"https://images.tidal.com/0/EMACGMAC/CAEQCCIDMjg2KgcjRkNBRUQ4MAQ?token=928d6aa411260fbf9d815e92d24adfc4ca929212","width":320,"height":320},{"size":"MEDIUM","url":"https://images.tidal.com/0/EIAFGIAF/CAEQCCIDMjg2KgcjRkNBRUQ4MAQ?token=2885575ffadef77a38df014dcbc7c801f8722da1","width":640,"height":640},{"size":"LARGE","url":"https://images.tidal.com/0/ENwLGNwL/CAEQCCIDMjg2KgcjRkNBRUQ4MAQ?token=77fe05ceaea7f931174accbf1432b70f62489f66","width":1500,"height":1500}],"type":"DISCOVERY_MIX","sortType":"POPULARITY","detailMixImages":[{"size":"SMALL","url":"https://images.tidal.com/0/EMACGMAC/CAEQCCIDMjg2KgcjRkNBRUQ4MAQ4AQ?token=c3e8754651f562100176be4a025263f008017e81","width":320,"height":320},{"size":"MEDIUM","url":"https://images.tidal.com/0/EIAFGIAF/CAEQCCIDMjg2KgcjRkNBRUQ4MAQ4AQ?token=36cc6f1f92d752e4389009e1a767eb92427f2158","width":640,"height":640},{"size":"LARGE","url":"https://images.tidal.com/0/ENwLGNwL/CAEQCCIDMjg2KgcjRkNBRUQ4MAQ4AQ?token=33f68348144676338fb6c3880f4a687ec4874907","width":1500,"height":1500}],"titleTextInfo":{"text":"My Daily Discovery","color":"#FCAED8"},"subtitleTextInfo":{"text":"Songs by new and familiar artists inspired by your listening. Updates every morning.","color":"#FCAED8"},"shortSubtitleTextInfo":{"text":"Created by TIDAL","color":"#FCAED8"},"description":{"text":null,"color":null},"contentBehavior":"UNRESTRICTED","userId":123456789,"atDate":"content_uploads"}}]},{"type":"HORIZONTAL_LIST","moduleId":"DAILY_MIXES","title":"Custom mixes","icons":[],"showQuickPlay":false,"viewAll":"home/pages/DAILY_MIXES/view-all","items":[{"type":"MIX","following":false,"data":{"type":"DAILY_MIX","countryCode":"WW","updated":1791879764042,"id":"000000000000000000000000000001","isStableId":false,"artifactIdType":"trackGroupId","mixImages":[{"size":"SMALL","url":"https://images.tidal.com/0/EMACGMACIKABKKAB/CAEaJGUxNDc2MWMyL2RhOGUvNDU0ZS85ZmY3LzI4OTQxYTVhZmYyYxokNDE2NTgxN2UvMGZiYS80NDViL2JlNjAvM2JjYTg1OWMwZDBjGiQ0ZGVlOGUxOC84ODc0LzRhYTgvYWZmOC82MzdlMDllMWMwZjkiATEqByNGNkIxNjIwAg?token=c77f20e39ae145070167fde05f0bb8e0b62bb747","width":320,"height":320},{"size":"MEDIUM","url":"https://images.tidal.com/0/EIAFGIAFIMACKMAC/CAEaJGUxNDc2MWMyL2RhOGUvNDU0ZS85ZmY3LzI4OTQxYTVhZmYyYxokNDE2NTgxN2UvMGZiYS80NDViL2JlNjAvM2JjYTg1OWMwZDBjGiQ0ZGVlOGUxOC84ODc0LzRhYTgvYWZmOC82MzdlMDllMWMwZjkiATEqByNGNkIxNjIwAg?token=c85b2fad7a1c76222ebc850b04ac9bffef2d64cf","width":640,"height":640},{"size":"LARGE","url":"https://images.tidal.com/0/ENwLGNwLIO4FKO4F/CAEaJGUxNDc2MWMyL2RhOGUvNDU0ZS85ZmY3LzI4OTQxYTVhZmYyYxokNDE2NTgxN2UvMGZiYS80NDViL2JlNjAvM2JjYTg1OWMwZDBjGiQ0ZGVlOGUxOC84ODc0LzRhYTgvYWZmOC82MzdlMDllMWMwZjkiATEqByNGNkIxNjIwAg?token=cc54baeaca8e26a77f8c2471b428bac806ad0fc1","width":1500,"height":1500}],"type":"DAILY_MIX","sortType":"POPULARITY","detailMixImages":[{"size":"SMALL","url":"https://images.tidal.com/0/EMACGMACIKABKKAB/CAEaJGUxNDc2MWMyL2RhOGUvNDU0ZS85ZmY3LzI4OTQxYTVhZmYyYxokNDE2NTgxN2UvMGZiYS80NDViL2JlNjAvM2JjYTg1OWMwZDBjGiQ0ZGVlOGUxOC84ODc0LzRhYTgvYWZmOC82MzdlMDllMWMwZjkiATEqByNGNkIxNjIwAjgB?token=0c50ba7c596f83207589fc5daf9fd80b19ef8f50","width":320,"height":320},{"size":"MEDIUM","url":"https://images.tidal.com/0/EIAFGIAFIMACKMAC/CAEaJGUxNDc2MWMyL2RhOGUvNDU0ZS85ZmY3LzI4OTQxYTVhZmYyYxokNDE2NTgxN2UvMGZiYS80NDViL2JlNjAvM2JjYTg1OWMwZDBjGiQ0ZGVlOGUxOC84ODc0LzRhYTgvYWZmOC82MzdlMDllMWMwZjkiATEqByNGNkIxNjIwAjgB?token=e05e8a40bea3d23ee932527d11eab59a73e89494","width":640,"height":640},{"size":"LARGE","url":"https://images.tidal.com/0/ENwLGNwLIO4FKO4F/CAEaJGUxNDc2MWMyL2RhOGUvNDU0ZS85ZmY3LzI4OTQxYTVhZmYyYxokNDE2NTgxN2UvMGZiYS80NDViL2JlNjAvM2JjYTg1OWMwZDBjGiQ0ZGVlOGUxOC84ODc0LzRhYTgvYWZmOC82MzdlMDllMWMwZjkiATEqByNGNkIxNjIwAjgB?token=241e86e4e268364a58074b5fc74aa251e18453ff","width":1500,"height":1500}],"titleTextInfo":{"text":"My Mix 1","color":"#F6B162"},"subtitleTextInfo":{"text":"101 Classical Music Masterpieces, Tomaso Albinoni, Franz Schubert and more","color":"#F6B162"},"shortSubtitleTextInfo":{"text":"Created by TIDAL","color":"#F6B162"},"description":{"text":"Essential Classical and Orchestral Masterworks","color":null},"contentBehavior":"UNRESTRICTED","artists":[{"artistId":4076842,"artistName":"101 Classical Music Masterpieces","artistImage":{"imageUuid":"e14761c2-da8e-454e-9ff7-28941a5aff2c","vibrantColor":"#f6b060"}},{"artistId":3901467,"artistName":"Tomaso Albinoni","artistImage":{"imageUuid":"4165817e-0fba-445b-be60-3bca859c0d0c","vibrantColor":"#e8d2a5"}},{"artistId":3603356,"artistName":"Franz Schubert","artistImage":{"imageUuid":"4dee8e18-8874-4aa8-aff8-637e09e1c0f9","vibrantColor":"#ba8f47"}},{"artistId":3917621,"artistName":"Frédéric Chopin","artistImage":{"imageUuid":"0947548c-80a7-4849-a0c1-2da86ca88aad","vibrantColor":"#b69949"}},{"artistId":27942,"artistName":"Sergei Rachmaninoff","artistImage":{"imageUuid":"e2eb7fa2-1d6e-44e5-b9af-0caaa0dbe249","vibrantColor":"#FFFFFF"}}],"mixNumber":1,"userId":123456789}},{"type":"MIX","following":false,"data":{"type":"TRACK_MIX","countryCode":"WW","updated":1791901564069,"id":"000000000000000000000000000000","isStableId":true,"artifactIdType":"trackGroupId","mixImages":[{"size":"SMALL","url":"https://images.tidal.com/0/EMACGMACIKABKKAB/CAEQBhokZjcyZGE0YzgvZDI0OC80ODM5L2IxYTEvM2YwYzQ1ZjExNTcwGiRlYTU2Y2NhYi9iMDljLzQ3MTMvOWY3Ny9mOGEwZDhiNTA0NmQaJGVjMjYyZDM5LzY4ZWYvNGI3YS9iOTRiLzAxNjI0NmEwOTM3MyILVHJhY2sgUmFkaW8iHlRoZSBCaXogR3Jhc3Nob3BwZXIgRXhwZXJpbWVudCoHI0VDRDk2RTAC?token=a514ea77abe9b4a0ea364e92e105840d29bab6c6","width":320,"height":320},{"size":"MEDIUM","url":"https://images.tidal.com/0/EIAFGIAFIMACKMAC/CAEQBhokZjcyZGE0YzgvZDI0OC80ODM5L2IxYTEvM2YwYzQ1ZjExNTcwGiRlYTU2Y2NhYi9iMDljLzQ3MTMvOWY3Ny9mOGEwZDhiNTA0NmQaJGVjMjYyZDM5LzY4ZWYvNGI3YS9iOTRiLzAxNjI0NmEwOTM3MyILVHJhY2sgUmFkaW8iHlRoZSBCaXogR3Jhc3Nob3BwZXIgRXhwZXJpbWVudCoHI0VDRDk2RTAC?token=c962df4df3a37c8873dfabce9ba7516773d12e79","width":640,"height":640},{"size":"LARGE","url":"https://images.tidal.com/0/ENwLGNwLIO4FKO4F/CAEQBhokZjcyZGE0YzgvZDI0OC80ODM5L2IxYTEvM2YwYzQ1ZjExNTcwGiRlYTU2Y2NhYi9iMDljLzQ3MTMvOWY3Ny9mOGEwZDhiNTA0NmQaJGVjMjYyZDM5LzY4ZWYvNGI3YS9iOTRiLzAxNjI0NmEwOTM3MyILVHJhY2sgUmFkaW8iHlRoZSBCaXogR3Jhc3Nob3BwZXIgRXhwZXJpbWVudCoHI0VDRDk2RTAC?token=ef8a148ea8219c2bbecd090206bc8a50d8a45eb5","width":1500,"height":1500}],"type":"TRACK_MIX","sortType":"POPULARITY","detailMixImages":[{"size":"SMALL","url":"https://images.tidal.com/0/EMACGMACIKABKKAB/CAEQBhokZjcyZGE0YzgvZDI0OC80ODM5L2IxYTEvM2YwYzQ1ZjExNTcwGiRlYTU2Y2NhYi9iMDljLzQ3MTMvOWY3Ny9mOGEwZDhiNTA0NmQaJGVjMjYyZDM5LzY4ZWYvNGI3YS9iOTRiLzAxNjI0NmEwOTM3MyILVHJhY2sgUmFkaW8iHlRoZSBCaXogR3Jhc3Nob3BwZXIgRXhwZXJpbWVudCoHI0VDRDk2RTACOAE?token=251b4ef222af83f86e96ecbdbeae228e2e89b57d","width":320,"height":320},{"size":"MEDIUM","url":"https://images.tidal.com/0/EIAFGIAFIMACKMAC/CAEQBhokZjcyZGE0YzgvZDI0OC80ODM5L2IxYTEvM2YwYzQ1ZjExNTcwGiRlYTU2Y2NhYi9iMDljLzQ3MTMvOWY3Ny9mOGEwZDhiNTA0NmQaJGVjMjYyZDM5LzY4ZWYvNGI3YS9iOTRiLzAxNjI0NmEwOTM3MyILVHJhY2sgUmFkaW8iHlRoZSBCaXogR3Jhc3Nob3BwZXIgRXhwZXJpbWVudCoHI0VDRDk2RTACOAE?token=d981f144fd6624e06d7de403a6b248575f76f0b1","width":640,"height":640},{"size":"LARGE","url":"https://images.tidal.com/0/ENwLGNwLIO4FKO4F/CAEQBhokZjcyZGE0YzgvZDI0OC80ODM5L2IxYTEvM2YwYzQ1ZjExNTcwGiRlYTU2Y2NhYi9iMDljLzQ3MTMvOWY3Ny9mOGEwZDhiNTA0NmQaJGVjMjYyZDM5LzY4ZWYvNGI3YS9iOTRiLzAxNjI0NmEwOTM3MyILVHJhY2sgUmFkaW8iHlRoZSBCaXogR3Jhc3Nob3BwZXIgRXhwZXJpbWVudCoHI0VDRDk2RTACOAE?token=47505a4290ad82d4987baf32c55b887c0afb2eda","width":1500,"height":1500}],"titleTextInfo":{"text":"The Biz Grasshopper Experiment","color":"#ECD96E"},"subtitleTextInfo":{"text":"Beastie Boys","color":"#ECD96E"},"shortSubtitleTextInfo":{"text":"Created by TIDAL","color":"#ECD96E"},"description":{"text":null,"color":null},"contentBehavior":"UNRESTRICTED","track":{"trackId":3049192,"trackGroup":"1ebcd13edda9554ba12d9761954b35d3","trackTitle":"The Biz Grasshopper Experiment","trackImage":{"imageUuid":"f72da4c8-d248-4839-b1a1-3f0c45f11570","vibrantColor":"#ecd96e"}},"artist":{"artistId":8963,"artistName":"Beastie Boys","artistImage":null},"isAI":false}}]},{"type":"TRACK_LIST","moduleId":"NEW_TRACK_SUGGESTIONS","title":"Recommended new tracks","icons":[],"items":[{"type":"TRACK","following":false,"data":{"id":565997839,"editable":false,"title":"Fantasy for Piano","album":{"id":565997836,"title":"Fantasy for Piano","version":null,"cover":"9b210b87-c1bc-4683-bd67-ffd1f269787a","vibrantColor":null,"videoCover":null,"releaseDate":"2026-10-13","upload":false,"ai":false},"artists":[{"id":13979,"name":"Brad Mehldau","handle":null,"picture":"610597fc-d5c4-4ee2-b3cd-1d41759f343d","main":true,"userId":null}],"version":null,"duration":672,"popularity":0,"doublePopularity":0.0,"trackNumber":1,"volumeNumber":1,"explicit":false,"replayGain":-4.59,"audioQuality":"LOSSLESS","allowStreaming":true,"streamStartDate":"2026-10-13T00","streamReady":true,"audioModes":["STEREO"],"mixes":null,"adSupportedStreamReady":true,"mediaMetadata":{"tags":["LOSSLESS","HIRES_LOSSLESS"]},"providerId":342,"providerName":"WMG","djReady":true,"stemReady":false,"payToStream":false,"genres":[{"id":225,"name":"Classical Crossover","parentId":17},{"id":17,"name":"Classical","parentId":null}],"audioAnalysisAttributes":{"keyScale":"MAJOR","bpm":"136.0","key":"F"},"upload":false,"accessType":"PUBLIC","spotlighted":false,"createdAt":"2026-09-30T21:28:31.61786Z","userId":null,"downloadReady":false,"priceCode":null,"ai":false,"isrc":"USNO12600302","peak":0.942267,"prePaywallPresentation":"PREVIEW","copyright":"℗ 2026 Nonesuch Records Inc.","trackGroup":"1f1bd1625f6fe2ba6bf8da10b38563c3","firstAvailable":"2026-10-12T11:00:00Z","linkedStereoIsrc":null,"validFrom":"2026-10-13T00:00:00","validUntil":"9999-01-01T00:00:00","inherited":null}}],"subtitle":null,"viewAll":"home/pages/NEW_TRACK_SUGGESTIONS/view-all"}]}"##;

    fn feed() -> HomeFeedWeb {
        serde_json::from_str(FEED).expect("the live feed shape should decode")
    }

    #[test]
    fn modules_decode_by_the_type_names_tidal_sends() {
        let feed = feed();
        assert!(matches!(feed.items[0], HomeItem::HomeShortcutList { .. }));
        assert!(matches!(feed.items[1], HomeItem::HomeHorizontalList { .. }));
        assert!(matches!(feed.items[2], HomeItem::HomeTrackList { .. }));
        let wire = serde_json::to_value(&feed.items[0]).expect("serialize");
        assert_eq!(wire["type"], "SHORTCUT_LIST");
    }

    #[test]
    fn mixes_with_a_repeated_type_key_are_mixes_not_unknown() {
        let feed = feed();
        let HomeItem::HomeShortcutList { inner } = &feed.items[0] else {
            unreachable!()
        };
        let HomeShortcutItem::Mix(shortcut) = &inner.items[0] else {
            panic!("shortcut mix decoded as {:?}", inner.items[0]);
        };
        assert_eq!(shortcut.data.mix_type, "DISCOVERY_MIX");
        assert_eq!(
            shortcut.data.title_text_info.text.as_deref(),
            Some("My Daily Discovery")
        );

        let HomeItem::HomeHorizontalList { inner } = &feed.items[1] else {
            unreachable!()
        };
        let [HomeListItem::Mix(daily), HomeListItem::Mix(track_mix)] = inner.items.as_slice()
        else {
            panic!("list mixes decoded as {:?}", inner.items);
        };
        assert_eq!(daily.data.mix_type, "DAILY_MIX");
        assert!(!daily.data.mix_images.is_empty());

        // Track mixes can name an artist without an image.
        assert_eq!(track_mix.data.mix_type, "TRACK_MIX");
        let artist = track_mix.data.artist.as_ref().expect("track mix artist");
        assert!(artist.artist_image.is_none());
    }

    #[test]
    fn tracks_without_mixes_are_tracks_not_unknown() {
        let feed = feed();
        let HomeItem::HomeTrackList { inner } = &feed.items[2] else {
            unreachable!()
        };
        let HomeListItem::Track(track) = &inner.items[0] else {
            panic!("track decoded as {:?}", inner.items[0]);
        };
        assert_eq!(track.data.title, "Fantasy for Piano");
        assert!(track.data.mixes.is_none());
    }
}
