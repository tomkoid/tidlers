use serde::{Deserialize, Serialize};
use crate::client::models::artist::Artist;

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
    pub seen: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowableActivity {
    pub activity_type: Option<String>,
    pub occurred_at: Option<String>,
    pub album: Option<ActivityFeedAlbum>,
    pub history_mix: Option<ActivityFeedHistoryMix>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityFeedAlbum {
    pub id: i64,
    pub title: String,
    #[serde(default)]
    pub artists: Vec<Artist>,
    #[serde(rename = "type")]
    pub album_type: Option<String>,
    pub cover: Option<String>,
    pub release_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityFeedHistoryMix {
    pub id: String,
    pub mix_type: Option<String>,
    pub title: Option<String>,
    pub sub_title: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::ActivityFeedResponse;

    #[test]
    fn deserializes_activity_feed_response() {
        let payload = r#"{
            "activities": [
                {
                    "followableActivity": {
                        "historyMix": {
                            "id": "008d5380fdf8c9111525f277ef3fde"
                        },
                        "occurredAt": "2026-05-01T00:00:00.000Z",
                        "activityType": "NEW_HISTORY_MIX"
                    },
                    "seen": true
                }
            ],
            "cursor": "next-page-cursor"
        }"#;

        let parsed: ActivityFeedResponse =
            serde_json::from_str(payload).expect("valid activity feed payload");
        assert_eq!(parsed.activities.len(), 1);
        assert_eq!(
            parsed.activities[0]
                .followable_activity
                .activity_type
                .as_deref(),
            Some("NEW_HISTORY_MIX")
        );
        assert!(parsed.activities[0].followable_activity.history_mix.is_some());
    }
}
