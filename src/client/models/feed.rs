use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityFeedResponse {
    pub activities: Vec<ActivityFeedItem>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityFeedItem {
    pub followable_activity: FollowableActivity,
    pub seen: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowableActivity {
    pub activity_type: Option<String>,
    pub occurred_at: Option<String>,
    #[serde(flatten)]
    pub payload: HashMap<String, serde_json::Value>,
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
    }
}
