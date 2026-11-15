use crate::client::models::IntoParamMap;

pub struct GetSaveForLaterItemsConfig {
    include_items: bool,
    include_items_cover_art: bool,
    include_items_artists: bool,
    include_items_profile_art: bool,
    include_items_thumbnail_art: bool,
    include_items_albums: bool,
    include_items_albums_cover_art: bool,
}

impl IntoParamMap for GetSaveForLaterItemsConfig {
    fn into_param_map(self) -> std::collections::HashMap<String, String> {
        let mut map = std::collections::HashMap::new();
        if self.include_items {
            map.insert("include".to_string(), "items".to_string());
        }
        if self.include_items_cover_art {
            map.insert("include".to_string(), "items.coverArt".to_string());
        }
        if self.include_items_artists {
            map.insert("include".to_string(), "items.artists".to_string());
        }
        if self.include_items_profile_art {
            map.insert("include".to_string(), "items.profileArt".to_string());
        }
        if self.include_items_thumbnail_art {
            map.insert("include".to_string(), "items.thumbnailArt".to_string());
        }
        if self.include_items_albums {
            map.insert("include".to_string(), "items.albums".to_string());
        }
        if self.include_items_albums_cover_art {
            map.insert("include".to_string(), "items.albums.coverArt".to_string());
        }

        map
    }
}

impl Default for GetSaveForLaterItemsConfig {
    fn default() -> Self {
        Self {
            include_items: true,
            include_items_cover_art: true,
            include_items_artists: true,
            include_items_profile_art: true,
            include_items_thumbnail_art: true,
            include_items_albums: true,
            include_items_albums_cover_art: true,
        }
    }
}
