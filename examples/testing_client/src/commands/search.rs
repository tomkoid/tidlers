use crate::args::SearchCommands;
use tidlers::{
    TidalClient,
    client::models::search::config::{SearchConfig, SearchSuggestionsConfig},
};

pub async fn execute(
    tidal: &mut TidalClient,
    query: String,
    command: SearchCommands,
) -> eyre::Result<()> {
    match command {
        SearchCommands::Direct => {
            let results = tidal
                .search(SearchConfig {
                    query: query.clone(),
                    limit: 10,
                    ..Default::default()
                })
                .await?;
            println!("{:#?}", results);
        }

        SearchCommands::Suggestions => {
            let results = tidal
                .search_suggestion(SearchSuggestionsConfig {
                    query: query.clone(),
                    ..Default::default()
                })
                .await?;
            println!("{:#?}", results);
        }
    }

    Ok(())
}
