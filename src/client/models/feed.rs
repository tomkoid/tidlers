use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityFeedResponse {
    pub activities: Vec<ActivityFeedItem>,
    pub cursor: Option<String>,
    pub stats: Option<ActivityFeedStats>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityFeedStats {
    pub total_not_seen_activities: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityFeedItem {
    pub followable_activity: FollowableActivity,
    #[serde(default)]
    pub seen: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowableActivity {
    pub activity_type: String,
    #[serde(default)]
    pub occurred_at: String,
    pub album: Option<ActivityFeedAlbumSource>,
    pub history_mix: Option<ActivityFeedHistoryMixSource>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityFeedAlbumSource {
    pub id: u64,
    pub title: String,
    pub cover: Option<String>,
    #[serde(default)]
    pub explicit: bool,
    pub release_date: Option<String>,
    pub audio_quality: Option<String>,
    #[serde(default)]
    pub number_of_tracks: u32,
    #[serde(default)]
    pub duration: u32,
    #[serde(default)]
    pub artists: Vec<ActivityFeedAlbumArtist>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityFeedAlbumArtist {
    pub id: Option<u64>,
    pub name: String,
    pub main: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityFeedHistoryMixSource {
    pub id: String,
    pub title_text_info: Option<ActivityFeedTextInfo>,
    pub sub_title_text_info: Option<ActivityFeedTextInfo>,
    pub images: Option<ActivityFeedImages>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityFeedTextInfo {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ActivityFeedImages {
    pub small: Option<ActivityFeedImage>,
    pub medium: Option<ActivityFeedImage>,
    pub large: Option<ActivityFeedImage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityFeedImage {
    pub width: u32,
    pub height: u32,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedActivity {
    pub item: FeedItem,
    pub occurred_at: String,
    pub seen: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FeedItem {
    AlbumRelease(FeedAlbum),
    HistoryMix(FeedHistoryMix),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedAlbum {
    pub id: String,
    pub title: String,
    pub artist_name: String,
    pub artist_id: Option<String>,
    pub num_tracks: u32,
    pub duration: u32,
    pub release_date: Option<String>,
    pub cover: Option<String>,
    pub explicit: bool,
    pub audio_quality: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedHistoryMix {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub image_url: Option<String>,
}

impl ActivityFeedResponse {
    pub fn into_activities(self) -> Vec<FeedActivity> {
        let mut activities = Vec::new();

        for item in self.activities {
            let followable = item.followable_activity;
            let occurred_at = followable.occurred_at;
            let seen = item.seen;

            let parsed_item = match followable.activity_type.as_str() {
                "NEW_ALBUM_RELEASE" => followable.album.and_then(parse_album_release),
                "NEW_HISTORY_MIX" => followable.history_mix.and_then(parse_history_mix),
                other => {
                    tracing::debug!("unknown feed activity type: {other}");
                    None
                }
            };

            if let Some(parsed_item) = parsed_item {
                activities.push(FeedActivity {
                    item: parsed_item,
                    occurred_at,
                    seen,
                });
            }
        }

        activities
    }
}

fn parse_album_release(album: ActivityFeedAlbumSource) -> Option<FeedItem> {
    let mut main_artist_names = Vec::new();
    let mut first_artist_id: Option<String> = None;

    for artist in &album.artists {
        if first_artist_id.is_none() {
            first_artist_id = artist.id.map(|id| id.to_string());
        }

        if artist.main.unwrap_or(true) {
            main_artist_names.push(artist.name.clone());
        }
    }

    let artist_name = if main_artist_names.is_empty() {
        "Unknown Artist".to_owned()
    } else {
        main_artist_names.join(", ")
    };

    Some(FeedItem::AlbumRelease(FeedAlbum {
        id: album.id.to_string(),
        title: album.title,
        artist_name,
        artist_id: first_artist_id,
        num_tracks: album.number_of_tracks,
        duration: album.duration,
        release_date: album.release_date,
        cover: album.cover,
        explicit: album.explicit,
        audio_quality: album.audio_quality,
    }))
}

fn parse_history_mix(mix: ActivityFeedHistoryMixSource) -> Option<FeedItem> {
    let title = mix
        .title_text_info
        .map(|v| v.text)
        .unwrap_or_else(|| "History Mix".to_owned());
    let subtitle = mix.sub_title_text_info.map(|v| v.text).unwrap_or_default();
    let image_url = mix.images.and_then(|v| v.small.map(|img| img.url));

    Some(FeedItem::HistoryMix(FeedHistoryMix {
        id: mix.id,
        title,
        subtitle,
        image_url,
    }))
}

#[cfg(test)]
mod tests {
    use super::{ActivityFeedResponse, FeedItem};

    #[test]
    fn parses_activity_feed_into_typed_items() {
        let payload = r#"{
            "activities": [
                {
                    "followableActivity": {
                        "activityType": "NEW_ALBUM_RELEASE",
                        "occurredAt": "2026-02-13T00:00:00.000Z",
                        "album": {
                            "id": 519171846,
                            "title": "The Memory Remains",
                            "artists": [
                                {"id": 8405, "name": "Metallica", "main": true},
                                {"id": 9060, "name": "Marianne Faithfull", "main": false}
                            ],
                            "cover": "94b2300b-ff93-4528-b48d-7d30c488da7f",
                            "explicit": false,
                            "releaseDate": "2026-04-28",
                            "audioQuality": "LOSSLESS",
                            "numberOfTracks": 1,
                            "duration": 319
                        }
                    },
                    "seen": true
                },
                {
                    "followableActivity": {
                        "activityType": "NEW_HISTORY_MIX",
                        "occurredAt": "2026-05-01T00:00:00.000Z",
                        "historyMix": {
                            "id": "008d5380fdf8c9111525f277ef3fde",
                            "titleTextInfo": {"text": "April 2026"},
                            "subTitleTextInfo": {"text": "Good Kid and more"},
                            "images": {
                                "SMALL": {"width": 267, "height": 267, "url": "https://img/s"}
                            }
                        }
                    },
                    "seen": false
                },
                {
                    "followableActivity": {
                        "activityType": "SOMETHING_ELSE",
                        "occurredAt": "2026-05-02T00:00:00.000Z"
                    },
                    "seen": false
                }
            ]
        }"#;

        let parsed: ActivityFeedResponse =
            serde_json::from_str(payload).expect("valid activity feed payload");
        let activities = parsed.into_activities();

        assert_eq!(activities.len(), 2);
        match &activities[0].item {
            FeedItem::AlbumRelease(album) => {
                assert_eq!(album.title, "The Memory Remains");
                assert_eq!(album.artist_name, "Metallica");
                assert_eq!(album.artist_id.as_deref(), Some("8405"));
            }
            other => panic!("expected album release, got {other:?}"),
        }
        match &activities[1].item {
            FeedItem::HistoryMix(mix) => {
                assert_eq!(mix.title, "April 2026");
                assert_eq!(mix.subtitle, "Good Kid and more");
            }
            other => panic!("expected history mix, got {other:?}"),
        }
    }
}
