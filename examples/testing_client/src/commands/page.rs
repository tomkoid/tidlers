use tidlers::TidalClient;

use crate::args::PageCommands;

/// Runs the page-related CLI subcommand.
pub async fn execute(tidal: &mut TidalClient, command: PageCommands) -> eyre::Result<()> {
    match command {
        PageCommands::Custom { slug } => {
            let page = tidal.get_page(slug.clone()).await?;
            println!("page '{slug}':\n{:?}", page);
        }
        PageCommands::Explore => {
            let page = tidal.get_page("explore").await?;
            println!("page 'explore':\n{:?}", page);
        }
        PageCommands::IndieRock => {
            let page = tidal.get_page("genre_indierock").await?;
            println!("page 'genre_indierock':\n{:?}", page);
        }
    }

    Ok(())
}
