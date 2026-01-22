pub struct UserUploadsInclude {
    pub albums: bool,
    pub albums_cover_art: bool,
    pub artists: bool,
    pub owners: bool,
    pub shares: bool,
    pub source_file: bool,
    pub track_stats: bool,
}

impl Default for UserUploadsInclude {
    fn default() -> Self {
        Self {
            albums: true,
            albums_cover_art: true,
            artists: true,
            owners: true,
            shares: true,
            source_file: true,
            track_stats: true,
        }
    }
}

impl UserUploadsInclude {
    pub fn to_api_params(&self) -> String {
        let mut includes = Vec::new();
        if self.albums {
            includes.push("albums");
        }
        if self.albums_cover_art {
            includes.push("albums.coverArt");
        }
        if self.artists {
            includes.push("artists");
        }
        if self.owners {
            includes.push("owners");
        }
        if self.shares {
            includes.push("shares");
        }
        if self.source_file {
            includes.push("sourceFile");
        }
        if self.track_stats {
            includes.push("trackStatistics");
        }

        includes.join(",")
    }
}
