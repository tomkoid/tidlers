use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub id: i64,
    pub title: String,
    pub cover: String,
    pub vibrant_color: Option<String>,
    pub video_cover: Option<String>,
    pub release_date: Option<String>,
}

impl Album {
    pub fn get_cover_url(&self, size_x: u32, size_y: u32) -> String {
        // split string by dashes
        let cover_parts: Vec<&str> = self.cover.split('-').collect();
        let mut cover_path = String::new();
        for part in cover_parts.iter() {
            cover_path.push_str(part);
        }

        let size = format!("{}x{}", size_x, size_y);
        format!("https://resources.tidal.com/images/{}/{}", cover_path, size)
    }
}
