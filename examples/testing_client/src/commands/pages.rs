use tidlers::TidalClient;

use crate::args::PagesCommands;

/// Runs the page-related CLI subcommand.
pub async fn execute(tidal: &mut TidalClient, command: PagesCommands) -> eyre::Result<()> {
    match command {
        PagesCommands::Custom { slug } => {
            let page = tidal.get_page(slug.clone()).await?;
            println!("page '{slug}':\n{:?}", page);
        }
        PagesCommands::Explore => {
            let page = tidal.get_page("explore").await?;
            println!("page 'explore':\n{:?}", page);
        }
        PagesCommands::IndieRock => {
            let page = tidal.get_page("genre_indierock").await?;
            println!("page 'genre_indierock':\n{:?}", page);
        }
    }

    Ok(())
}
