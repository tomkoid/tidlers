use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// One page of the user's followed artists.
///
/// The collection is paged by cursor: pass [`cursor`](Self::cursor) to
/// [`get_collection_artists`](crate::client::TidalClient::get_collection_artists)
/// to get the next page. It is `None` on the last page.
#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionArtistsResponse {
    #[serde(rename = "lastModifiedAt")]
    pub last_modified_at: String,
    pub items: Vec<CollectionArtistEntry>,
    /// Number of followed artists across all pages.
    #[serde(rename = "totalNumberOfItems")]
    pub total_number_of_items: u64,
    /// Opaque position of the next page; `None` on the last page.
    pub cursor: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionArtistEntry {
    pub trn: String,
    #[serde(rename = "itemType")]
    pub item_type: String,
    #[serde(rename = "addedAt")]
    pub added_at: String,
    #[serde(rename = "lastModifiedAt")]
    pub last_modified_at: String,
    pub name: String,
    pub parent: Option<String>,
    pub data: CollectionArtistDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionArtistDetails {
    pub id: i64,
    pub name: String,
    pub picture: Option<String>,
    pub popularity: i64,
    #[serde(rename = "doublePopularity")]
    pub double_popularity: f64,
    #[serde(rename = "artistTypes")]
    pub artist_types: Vec<String>,
    #[serde(rename = "artistRoles")]
    pub artist_roles: Vec<CollectionArtistRoleEntry>,
    pub mixes: Option<HashMap<String, String>>,
    #[serde(rename = "vibrantColor")]
    pub vibrant_color: Option<String>,
    #[serde(rename = "selectedAlbumCoverFallback")]
    pub selected_album_cover_fallback: Option<String>,
    pub handle: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: Option<i64>,
    pub url: String,
    pub trn: String,
    #[serde(rename = "itemType")]
    pub item_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionArtistRoleEntry {
    #[serde(rename = "categoryId")]
    pub category_id: i64,
    pub category: String,
}

#[cfg(test)]
mod tests {
    use super::CollectionArtistsResponse;

    /// Shape captured from TIDAL (limit 2): more pages follow, so it has a
    /// `cursor`. Follow timestamps replaced with placeholders.
    const FIRST_PAGE: &str = r##"{
        "lastModifiedAt": "2026-01-01T00:00:00.000+0000",
        "items": [
            {
                "trn": "trn:artist:6610450",
                "itemType": "ARTIST",
                "addedAt": "2026-01-01T00:00:00.000+0000",
                "lastModifiedAt": "2026-01-01T00:00:00.000+0000",
                "name": "Joey Alexander",
                "parent": null,
                "data": {
                    "artistRoles": [
                        {
                            "category": "Artist",
                            "categoryId": -1
                        },
                        {
                            "category": "Songwriter",
                            "categoryId": 2
                        },
                        {
                            "category": "Performer",
                            "categoryId": 11
                        },
                        {
                            "category": "Producer",
                            "categoryId": 1
                        },
                        {
                            "category": "Production team",
                            "categoryId": 10
                        },
                        {
                            "category": "Misc",
                            "categoryId": 99
                        }
                    ],
                    "artistTypes": [
                        "ARTIST",
                        "CONTRIBUTOR"
                    ],
                    "artworkId": null,
                    "cashAppOnboarded": false,
                    "contributionsEnabled": false,
                    "doublePopularity": 0.5933598590559294,
                    "handle": null,
                    "id": 6610450,
                    "mixes": {
                        "ARTIST_MIX": "000524f33c2181bf64fe473e0f95a5"
                    },
                    "name": "Joey Alexander",
                    "ownerType": "LABEL",
                    "picture": "72712db1-ddb6-448c-a686-38a31e4ed76e",
                    "popularity": 59,
                    "selectedAlbumCoverFallback": null,
                    "spotlighted": false,
                    "userId": null,
                    "vibrantColor": "#cf6658",
                    "itemType": "ARTIST",
                    "trn": "trn:artist:6610450",
                    "url": "http://www.tidal.com/artist/6610450"
                }
            },
            {
                "trn": "trn:artist:3575706",
                "itemType": "ARTIST",
                "addedAt": "2026-01-01T00:00:00.000+0000",
                "lastModifiedAt": "2026-01-01T00:00:00.000+0000",
                "name": "The Hacker",
                "parent": null,
                "data": {
                    "artistRoles": [
                        {
                            "category": "Artist",
                            "categoryId": -1
                        },
                        {
                            "category": "Engineer",
                            "categoryId": 3
                        },
                        {
                            "category": "Songwriter",
                            "categoryId": 2
                        },
                        {
                            "category": "Producer",
                            "categoryId": 1
                        },
                        {
                            "category": "Performer",
                            "categoryId": 11
                        },
                        {
                            "category": "Production team",
                            "categoryId": 10
                        }
                    ],
                    "artistTypes": [
                        "ARTIST",
                        "CONTRIBUTOR"
                    ],
                    "artworkId": null,
                    "cashAppOnboarded": false,
                    "contributionsEnabled": false,
                    "doublePopularity": 0.5302478851884551,
                    "handle": null,
                    "id": 3575706,
                    "mixes": {
                        "ARTIST_MIX": "000540386e98f289b30c11c6e353fc"
                    },
                    "name": "The Hacker",
                    "ownerType": "LABEL",
                    "picture": null,
                    "popularity": 53,
                    "selectedAlbumCoverFallback": "1ba874c6-cdba-442a-917b-2047aa0936f9",
                    "spotlighted": false,
                    "userId": null,
                    "vibrantColor": null,
                    "itemType": "ARTIST",
                    "trn": "trn:artist:3575706",
                    "url": "http://www.tidal.com/artist/3575706"
                }
            }
        ],
        "totalNumberOfItems": 3,
        "cursor": "eyJpZCI6IjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDM2OGY5YSIsIm5hbWUiOiJUaGUgSGFja2VyIiwiYWRkZWRBdCI6IjIwMjYtMDEtMDFUMDA6MDA6MDAuMDAwWiIsInVwZGF0ZWRBdCI6bnVsbCwiaXRlbVR5cGUiOiJGQVZPUklURV9BUlRJU1QiLCJhcnRpc3ROYW1lIjpudWxsLCJyZWxlYXNlZEF0IjpudWxsLCJtaXhUeXBlIjpudWxsfQ"
    }"##;

    /// The following, last page: no `cursor`.
    const LAST_PAGE: &str = r##"{
        "lastModifiedAt": "2026-01-01T00:00:00.000+0000",
        "items": [
            {
                "trn": "trn:artist:4847",
                "itemType": "ARTIST",
                "addedAt": "2026-01-01T00:00:00.000+0000",
                "lastModifiedAt": "2026-01-01T00:00:00.000+0000",
                "name": "Prince",
                "parent": null,
                "data": {
                    "artistRoles": [
                        {
                            "category": "Artist",
                            "categoryId": -1
                        },
                        {
                            "category": "Songwriter",
                            "categoryId": 2
                        },
                        {
                            "category": "Performer",
                            "categoryId": 11
                        },
                        {
                            "category": "Producer",
                            "categoryId": 1
                        },
                        {
                            "category": "Production team",
                            "categoryId": 10
                        },
                        {
                            "category": "Engineer",
                            "categoryId": 3
                        },
                        {
                            "category": "Misc",
                            "categoryId": 99
                        }
                    ],
                    "artistTypes": [
                        "ARTIST",
                        "CONTRIBUTOR"
                    ],
                    "artworkId": null,
                    "cashAppOnboarded": false,
                    "contributionsEnabled": false,
                    "doublePopularity": 0.8770956517092108,
                    "handle": null,
                    "id": 4847,
                    "mixes": {
                        "ARTIST_MIX": "0000e35689c18b5bfdc62407d226ed"
                    },
                    "name": "Prince",
                    "ownerType": "MIXED",
                    "picture": "f61a6f16-dc73-4645-bd5d-f2706c042769",
                    "popularity": 88,
                    "selectedAlbumCoverFallback": null,
                    "spotlighted": false,
                    "userId": 193189460,
                    "vibrantColor": "#bd0042",
                    "itemType": "ARTIST",
                    "trn": "trn:artist:4847",
                    "url": "http://www.tidal.com/artist/4847"
                }
            }
        ],
        "totalNumberOfItems": 3
    }"##;

    #[test]
    fn pages_carry_the_cursor_to_the_next_one() {
        let first: CollectionArtistsResponse =
            serde_json::from_str(FIRST_PAGE).expect("first page");
        let last: CollectionArtistsResponse = serde_json::from_str(LAST_PAGE).expect("last page");

        assert_eq!(first.total_number_of_items, 3);
        assert_eq!(first.items.len(), 2);
        assert!(first.cursor.as_deref().is_some_and(|c| !c.is_empty()));

        assert_eq!(last.total_number_of_items, 3);
        assert_eq!(last.items.len(), 1);
        assert_eq!(last.cursor, None);
    }

    #[test]
    fn entries_keep_their_artist_details() {
        let first: CollectionArtistsResponse =
            serde_json::from_str(FIRST_PAGE).expect("first page");
        let last: CollectionArtistsResponse = serde_json::from_str(LAST_PAGE).expect("last page");

        let joey = &first.items[0].data;
        assert_eq!((joey.id, joey.name.as_str()), (6610450, "Joey Alexander"));
        assert_eq!(
            joey.picture.as_deref(),
            Some("72712db1-ddb6-448c-a686-38a31e4ed76e")
        );
        assert_eq!(joey.artist_roles[0].category, "Artist");

        // Artists without a picture, or linked to a TIDAL user, are common.
        assert_eq!(first.items[1].data.picture, None);
        assert!(last.items[0].data.user_id.is_some());
    }
}
