pub const DEFAULT_SIZE_PX: u32 = 320;

/// Returns CDN URL for cover art with default size of (DEFAULT_SIZE_PX)x(DEFAULT_SIZE_PX)
pub fn uuid_to_url(uuid: &str) -> String {
    uuid_to_url_with_size(uuid, DEFAULT_SIZE_PX)
}

/// Returns CDN URL for cover art with a specific size in pixels
pub fn uuid_to_url_with_size(uuid: &str, size_px: u32) -> String {
    format!(
        "https://resources.tidal.com/images/{}/{size_px}x{size_px}.jpg",
        uuid.replace('-', "/")
    )
}
