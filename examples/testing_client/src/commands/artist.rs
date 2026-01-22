use crate::args::ArtistCommands;
use tidlers::TidalClient;

pub async fn execute(
    tidal: &mut TidalClient,
    artist_id: String,
    command: ArtistCommands,
) -> eyre::Result<()> {
    match command {
        ArtistCommands::Info => {
            let info = tidal.get_artist(artist_id).await?;
            println!("{:#?}", info);
        }

        ArtistCommands::TopTracks => {
            let tracks = tidal.get_artist_tracks(artist_id, Some(2), None).await?;
            println!("{:#?}", tracks);
        }

        ArtistCommands::Bio => {
            let bio = tidal.get_artist_bio(artist_id).await?;
            println!("{:#?}", bio);
        }

        ArtistCommands::Links => {
            let links = tidal.get_artist_links(artist_id).await?;
            println!("{:#?}", links);
        }

        ArtistCommands::Albums => {
            let albums = tidal.get_artist_albums(artist_id, None, None).await?;
            println!("{:#?}", albums);
        }

        ArtistCommands::Mix => {
            let mix = tidal.get_artist_mix(artist_id).await?;
            println!("{:#?}", mix);
        }

        ArtistCommands::Videos => {
            let videos = tidal.get_artist_videos(artist_id, None, None).await?;
            println!("{:#?}", videos);
        }
    }

    Ok(())
}
