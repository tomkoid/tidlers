use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResponse {
    pub self_link: Option<String>,
    pub id: String,
    pub title: String,
    pub rows: Vec<PageRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageRow {
    pub modules: Vec<PageModule>,
}

/// A page module with metadata and content appropriate to its type.
///
/// Promotions carry top-level `items`; ordinary lists use `pagedList`.
/// Metadata may be absent or null. Unknown module types and fields remain
/// available to callers rather than preventing the rest of a page from loading.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageModule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub page_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_more: Option<PageModuleShowMore>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paged_list: Option<PageModulePagedList>,
    /// Direct module items, such as `FEATURED_PROMOTIONS` cards.
    /// Independent of `paged_list`: either or both collections may be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_format: Option<String>,

    /// The width of the module on official Tidal clients, when specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    /// Type-specific metadata and payloads not described by the common fields.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageModuleShowMore {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_path: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageModulePagedList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_number_of_items: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_api_path: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[cfg(test)]
mod tests {
    use super::{PageModule, PageResponse};
    use serde_json::{Value, json};

    fn page(modules: Vec<Value>) -> PageResponse {
        serde_json::from_value(json!({
            "id": "explore", "title": "Explore",
            "rows": [{"modules": modules}]
        }))
        .expect("page should parse")
    }

    #[test]
    fn sparse_promotions_preserve_top_level_items() {
        let cards = json!([{
            "header": "Example album", "type": "ALBUM", "artifactId": "123",
            "imageId": "cover-id", "futureProperty": {"enabled": true}
        }]);
        let response = page(vec![json!({
            "type": "FEATURED_PROMOTIONS", "items": cards
        })]);
        let module = &response.rows[0].modules[0];
        assert_eq!(module.page_type, "FEATURED_PROMOTIONS");
        assert!(module.id.is_none());
        assert!(module.title.is_none());
        assert!(module.description.is_none());
        assert!(module.width.is_none());
        assert!(module.paged_list.is_none());
        assert_eq!(serde_json::to_value(&module.items).unwrap(), cards);
        assert_eq!(
            serde_json::to_value(&response).unwrap()["rows"][0]["modules"][0]["items"],
            cards
        );
    }

    #[test]
    fn mixed_page_preserves_promos_links_and_ordinary_lists() {
        let modules = vec![
            json!({"type": "FEATURED_PROMOTIONS", "title": "Featured",
                "items": [{"header": "Example", "artifactId": "1"}]}),
            json!({"type": "PAGE_LINKS_CLOUD", "title": "Genres",
                "pagedList": {"items": [{"text": "R&B / Soul", "apiPath": "pages/genre_rnb"}]}}),
            json!({"type": "ALBUM_LIST", "title": "Albums", "description": "Albums to explore", "width": 12,
                "pagedList": {"items": [{"id": 123, "title": "Album", "artist": null}],
                    "limit": 10, "offset": 0, "totalNumberOfItems": 1}}),
            json!({"type": "PLAYLIST_LIST", "pagedList": {"items": [{"uuid": "playlist-1", "title": "Playlist"}]}}),
            json!({"type": "ARTIST_LIST", "pagedList": {"items": [{"id": 456, "name": "Artist"}]}}),
        ];
        let response = page(modules.clone());
        assert_eq!(response.rows[0].modules.len(), modules.len());
        assert_eq!(
            serde_json::to_value(&response).unwrap()["rows"][0]["modules"],
            json!(modules)
        );

        let album = &response.rows[0].modules[2];
        assert_eq!(album.description.as_deref(), Some("Albums to explore"));
        assert_eq!(album.width, Some(12));
        let list = album.paged_list.as_ref().unwrap();
        assert_eq!(list.limit, Some(10));
        assert_eq!(list.offset, Some(0));
        assert_eq!(list.total_number_of_items, Some(1));
    }

    #[test]
    fn null_metadata_and_collections_are_optional() {
        let response = page(vec![json!({
            "id": null, "type": "PROMO_BANNER", "title": null,
            "description": null, "width": null, "layout": null,
            "showMore": null, "pagedList": null, "items": null,
            "preTitle": null, "listFormat": null
        })]);
        let module = &response.rows[0].modules[0];
        assert!(module.id.is_none());
        assert!(module.title.is_none());
        assert!(module.description.is_none());
        assert!(module.width.is_none());
        assert!(module.items.is_none());
        assert!(module.paged_list.is_none());
    }

    #[test]
    fn unknown_module_payload_survives_without_blocking_known_modules() {
        let unknown = json!({
            "type": "FUTURE_BANNER_V2",
            "payload": {"cta": {"apiPath": "pages/another_page"}, "image": null},
            "flags": ["new"]
        });
        let response = page(vec![
            unknown.clone(),
            json!({"type": "ALBUM_LIST", "pagedList": {"items": []}}),
        ]);
        let module = &response.rows[0].modules[0];
        assert_eq!(module.page_type, "FUTURE_BANNER_V2");
        assert_eq!(module.extra["payload"], unknown["payload"]);
        assert_eq!(serde_json::to_value(module).unwrap(), unknown);
        assert_eq!(response.rows[0].modules[1].page_type, "ALBUM_LIST");
    }

    #[test]
    fn both_item_locations_are_preserved_independently() {
        let module = json!({
            "type": "HYBRID_LIST",
            "items": [{"id": "direct"}],
            "pagedList": {"items": [{"id": "paged"}]}
        });
        let response = page(vec![module.clone()]);
        let parsed = &response.rows[0].modules[0];
        assert_eq!(parsed.items.as_ref().unwrap()[0]["id"], "direct");
        assert_eq!(
            parsed.paged_list.as_ref().unwrap().items.as_ref().unwrap()[0]["id"],
            "paged"
        );
        assert_eq!(serde_json::to_value(parsed).unwrap(), module);
    }

    #[test]
    fn paged_list_can_carry_only_a_data_path() {
        let module = json!({
            "type": "ALBUM_LIST",
            "pagedList": {"dataApiPath": "albums/123/items", "cursor": "next-page"},
            "showMore": {"apiPath": "pages/all_albums", "customFlag": true}
        });
        let response = page(vec![module.clone()]);
        let parsed = &response.rows[0].modules[0];
        let list = parsed.paged_list.as_ref().unwrap();
        assert!(list.items.is_none());
        assert!(list.limit.is_none());
        assert!(list.offset.is_none());
        assert!(list.total_number_of_items.is_none());
        assert_eq!(list.data_api_path.as_deref(), Some("albums/123/items"));
        assert_eq!(serde_json::to_value(parsed).unwrap(), module);
    }

    #[test]
    fn nested_pagination_metadata_accepts_nulls() {
        let response = page(vec![json!({
            "type": "ALBUM_LIST",
            "pagedList": {"items": null, "limit": null, "offset": null,
                "totalNumberOfItems": null, "lines": null, "dataApiPath": null},
            "showMore": {"title": null, "apiPath": null}
        })]);
        let module = &response.rows[0].modules[0];
        let list = module.paged_list.as_ref().unwrap();
        assert!(list.items.is_none());
        assert!(list.limit.is_none());
        assert!(list.offset.is_none());
        assert!(list.total_number_of_items.is_none());
        assert!(module.show_more.as_ref().unwrap().title.is_none());
    }

    #[test]
    fn empty_page_and_explicit_empty_collections_remain_valid() {
        assert!(page(vec![]).rows[0].modules.is_empty());
        let response = page(vec![
            json!({"type": "EMPTY_MODULE", "items": [], "pagedList": {"items": []}}),
        ]);
        let module = &response.rows[0].modules[0];
        assert_eq!(module.items.as_deref(), Some(&[][..]));
        assert_eq!(
            module.paged_list.as_ref().unwrap().items.as_deref(),
            Some(&[][..])
        );
    }

    #[test]
    fn malformed_envelopes_and_known_field_types_still_fail() {
        assert!(serde_json::from_value::<PageResponse>(json!({"status": 401})).is_err());
        assert!(serde_json::from_value::<PageModule>(json!({"title": "Missing type"})).is_err());
        for module in [
            json!({"type": "ALBUM_LIST", "width": "not a number"}),
            json!({"type": "ALBUM_LIST", "pagedList": "not a list object"}),
            json!({"type": "FEATURED_PROMOTIONS", "items": "not an array"}),
        ] {
            assert!(serde_json::from_value::<PageModule>(module).is_err());
        }
    }
}
