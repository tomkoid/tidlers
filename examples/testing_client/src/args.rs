use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug)]
pub enum Commands {
    /// Show user information
    UserInfo,

    /// List user's playlists
    Playlists,

    /// Show details of a specific playlist
    Playlist {
        /// Playlist ID
        playlist_id: String,
    },

    /// Show user's collection
    Collection,

    /// Show user's activity
    Activity,

    /// Show user's subscription details
    Subscription,

    /// Show user's arrival mixes
    ArrivalMixes,

    /// Show details of a specific track
    Track {
        /// Track ID
        track_id: String,
    },

    /// Show details of a specific album
    Album {
        /// Album ID
        album_id: String,
    },

    /// Logout the user
    Logout,
}
