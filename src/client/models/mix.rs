use crate::client::models::{responses::ApiLinks, track::Track};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TrackMixResponse {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Mixes {
    pub track_mix: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrivalMixResource {
    pub id: String,
    #[serde(rename = "type")]
    pub data_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MixItemsResponse {
    pub limit: u32,
    pub offset: u32,
    pub total_number_of_items: u32,
    #[serde(deserialize_with = "deserialize_mix_items")]
    pub items: Vec<Track>,
}

pub type ArrivalMixResourceLinks = ApiLinks;

#[derive(Debug, Deserialize)]
struct MixItemEnvelope {
    item: Track,
}

fn deserialize_mix_items<'de, D>(deserializer: D) -> Result<Vec<Track>, D::Error>
where
    D: Deserializer<'de>,
{
    let envelopes = Vec::<MixItemEnvelope>::deserialize(deserializer)?;
    Ok(envelopes.into_iter().map(|entry| entry.item).collect())
}

#[cfg(test)]
mod tests {
    use super::MixItemsResponse;

    #[test]
    fn deserializes_mix_items_from_item_envelopes() {
        let payload = r#"{
            "limit": 1,
            "offset": 0,
            "totalNumberOfItems": 1,
            "items": [
                {
                    "item": {
                        "id": 304775901,
                        "title": "Thinking Spot",
                        "duration": 190,
                        "replayGain": -2.93,
                        "peak": 0.968414,
                        "allowStreaming": true,
                        "streamReady": true,
                        "payToStream": false,
                        "adSupportedStreamReady": true,
                        "djReady": true,
                        "stemReady": false,
                        "streamStartDate": "2023-07-13T00:00:00.000+0000",
                        "premiumStreamingOnly": false,
                        "trackNumber": 8,
                        "volumeNumber": 1,
                        "version": null,
                        "popularity": 23,
                        "copyright": "2022 Lofi Records",
                        "bpm": 73.0,
                        "key": "Bb",
                        "keyScale": "MAJOR",
                        "url": "http://www.tidal.com/track/304775901",
                        "isrc": "GBKQU2223264",
                        "editable": false,
                        "explicit": false,
                        "audioQuality": "LOSSLESS",
                        "audioModes": ["STEREO"],
                        "mediaMetadata": {"tags": ["LOSSLESS"]},
                        "upload": false,
                        "accessType": "PUBLIC",
                        "spotlighted": false,
                        "artist": {
                            "id": 9130815,
                            "name": "xander.",
                            "type": "MAIN"
                        },
                        "artists": [
                            {
                                "id": 9130815,
                                "name": "xander.",
                                "type": "MAIN"
                            }
                        ],
                        "album": {
                            "id": 304775893,
                            "title": "Forest Of Dreams",
                            "cover": "059a3281-4f34-464e-9f34-dbbccdc2b43b"
                        },
                        "mixes": {"TRACK_MIX": "0018b0352e33252ee96a692e1971f4"}
                    },
                    "type": "track"
                }
            ]
        }"#;

        let parsed: MixItemsResponse = serde_json::from_str(payload).expect("valid mix payload");
        assert_eq!(parsed.items.len(), 1);
        assert_eq!(parsed.items[0].id, 304775901);
    }
}
