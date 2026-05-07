use clap::Parser;
use tidlers::client::models::{
    collection::favorites::FavoriteResourceType, playback::AudioQuality,
};

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,

    /// Enable TidalClient debug mode for more verbose output
    #[clap(short, long, default_value_t = false)]
    pub debug: bool,

    /// Trace requests and responses for the command being executed
    #[clap(short, long, default_value_t = false)]
    pub trace: bool,
}

#[derive(Parser, Debug)]
pub enum Commands {
    /// Show user's own information
    UserInfo,

    /// Show user information
    User {
        /// User ID
        user_id: String,
    },

    /// List user's playlists
    Playlists,

    /// Show user's collection
    Collection {
        #[clap(subcommand)]
        command: CollectionCommands,
    },

    /// Show user's activity
    Activity {
        /// Year of activity to show
        #[clap(short, long, default_value = None)]
        year: Option<i32>,

        /// Month of activity to show
        #[clap(short, long, default_value = None)]
        month: Option<u32>,
    },

    /// Show user's subscription details
    Subscription,

    /// Mixes
    Mix {
        #[clap(subcommand)]
        command: MixCommands,
    },

    /// Show user's home feed
    Home,

    /// Show user's uploads
    Uploads,

    /// Show details of a specific track
    Track {
        #[clap(short, long, default_value_t = ArgAudioQuality::High, value_enum)]
        quality: ArgAudioQuality,

        /// Track ID
        track_id: String,
    },

    /// Show details of a specific album
    Album {
        #[clap(subcommand)]
        command: AlbumCommands,

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

    /// Search
    Search {
        #[clap(subcommand)]
        command: SearchCommands,

        /// Query
        query: String,
    },

    /// Logout the user
    Logout,
}

#[derive(Parser, Debug, Clone)]
pub enum SearchCommands {
    /// Show search results
    Direct,

    /// Show search suggestions for query
    Suggestions,
}

#[derive(Parser, Debug, Clone)]
pub enum AlbumCommands {
    /// Show album info
    Info,

    /// Show album items
    Items,

    /// Show album credits
    Credits,

    /// Show album items credits
    ItemsCredits,

    /// Show album review
    Review,
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
    /// Show artist's albums
    Albums,
    /// Show artist's mix
    Mix,
    /// Show artist's videos
    Videos,
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

    /// Add a type to favorites
    Favorite {
        /// Type to add to favorites (tracks or albums)
        #[clap(short = 't', long, value_enum)]
        resource_type: ArgFavoriteResourceType,

        /// ID of the resource to add to favorites
        id: String,

        /// Remove from favorites instead of adding
        #[clap(short, long, default_value_t = false)]
        remove: bool,
    },

    /// Show favorite tracks
    Tracks {
        /// Limit number of favorites to show (default 100)
        #[clap(short, long, default_value = "100")]
        limit: u32,

        /// Offset for favorites to show (default 0)
        #[clap(short, long, default_value = "0")]
        offset: u32,
    },

    /// Show favorite albums
    Albums {
        /// Limit number of favorites to show (default 100)
        #[clap(short, long, default_value = "100")]
        limit: u32,

        /// Offset for favorites to show (default 0)
        #[clap(short, long, default_value = "0")]
        offset: u32,
    },
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
    Remove {
        /// Folder ID (without the trn:folder prefix)
        id: String,
    },
    Flattened,
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
    RecommendationsItems {
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

#[derive(Parser, Debug, Clone)]
pub enum MixCommands {
    /// Show items of a specific mix
    Items {
        /// Mix ID
        mix_id: String,

        /// Limit number of items to show (default 100)
        #[clap(short, long, default_value = "100")]
        limit: u32,

        /// Offset for items to show (default 0)
        #[clap(short, long, default_value = "0")]
        offset: u32,
    },

    /// Show arrival mixes
    ArrivalMixes,
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

#[derive(clap::ValueEnum, Clone, Default, Debug)]
pub enum ArgFavoriteResourceType {
    #[clap(name = "tracks")]
    #[default]
    Tracks,
    #[clap(name = "albums")]
    Albums,
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

impl ArgFavoriteResourceType {
    pub fn to_favorite_resource_type(&self) -> FavoriteResourceType {
        match self {
            ArgFavoriteResourceType::Tracks => FavoriteResourceType::Tracks,
            ArgFavoriteResourceType::Albums => FavoriteResourceType::Albums,
        }
    }
}
