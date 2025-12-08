use clap::Parser;
use tidlers::client::models::playback::AudioQuality;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,

    #[clap(short, long, default_value_t = false)]
    pub debug: bool,
}

#[derive(Parser, Debug)]
pub enum Commands {
    /// Show user information
    UserInfo,

    /// List user's playlists
    Playlists,

    /// Show user's collection
    Collection {
        #[clap(subcommand)]
        command: CollectionCommands,
    },

    /// Show user's activity
    Activity,

    /// Show user's subscription details
    Subscription,

    /// Show user's arrival mixes
    ArrivalMixes,

    /// Show details of a specific track
    Track {
        #[clap(short, long, default_value_t = ArgAudioQuality::High, value_enum)]
        quality: ArgAudioQuality,

        /// Track ID
        track_id: String,
    },

    /// Show details of a specific album
    Album {
        /// Album ID
        album_id: String,
    },

    /// Show details of a specific artist
    Artist {
        #[clap(subcommand)]
        command: ArtistCommands,

        /// Artist ID
        artist_id: String,
    },

    /// Logout the user
    Logout,
}

#[derive(Parser, Debug, Clone)]
pub enum ArtistCommands {
    /// Show artist's info
    Info,
    /// Show artist's top tracks
    TopTracks,
    /// Show artist's bio
    Bio,
    /// Show artist's links
    Links,
    // /// Show artist's albums
    // Albums,
    // /// Show artist's related artists
    // Related,
}

#[derive(Parser, Debug, Clone)]
pub enum CollectionCommands {
    /// Show details of a specific playlist
    Playlist {
        #[clap(subcommand)]
        command: PlaylistCommands,
    },

    Folder {
        #[clap(subcommand)]
        command: FolderCommands,
    },

    /// Show collection artists
    Artists,
    /// Show collection favorites
    Favorites,
}

#[derive(Parser, Debug, Clone)]
pub enum FolderCommands {
    Create {
        /// Folder name
        name: String,

        /// Parent ID to create the folder in
        #[clap(short, long)]
        parent_id: Option<String>,
    },
}

#[derive(Parser, Debug, Clone)]
pub enum PlaylistCommands {
    /// Show details of a specific playlist
    Info {
        /// Playlist UUID
        playlist_id: String,
    },
    /// Show items of a specific playlist
    Items {
        /// Playlist UUID
        playlist_id: String,
    },
    Create {
        /// Playlist name
        name: String,
        /// Playlist description
        #[clap(short, long, default_value = "")]
        description: String,
        /// Folder ID to create the playlist in
        #[clap(short, long)]
        folder_id: Option<String>,
        /// Sharing level (PUBLIC or PRIVATE)
        #[clap(short, long)]
        sharing_level: Option<ArgSharingLevel>,
    },
    /// List playlists
    List,
    /// List public playlists
    ListPublic,
}

#[derive(clap::ValueEnum, Clone, Default, Debug)]
pub enum ArgSharingLevel {
    #[clap(name = "PUBLIC")]
    Public,
    #[clap(name = "PRIVATE")]
    #[default]
    Private,
}

#[derive(clap::ValueEnum, Clone, Default, Debug)]
pub enum ArgAudioQuality {
    #[clap(name = "low")]
    Low,
    #[clap(name = "high")]
    #[default]
    High,
    #[clap(name = "lossless")]
    Lossless,
    #[clap(name = "hires")]
    HiRes,
}

impl ArgAudioQuality {
    pub fn to_api_quality(&self) -> AudioQuality {
        match self {
            ArgAudioQuality::Low => AudioQuality::Low,
            ArgAudioQuality::High => AudioQuality::High,
            ArgAudioQuality::Lossless => AudioQuality::Lossless,
            ArgAudioQuality::HiRes => AudioQuality::HiRes,
        }
    }
}
