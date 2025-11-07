use color_eyre::eyre::Result;
use tidlers::client::tidal::TidalClient;

use crate::auth::handle_auth;

mod auth;
mod oauth_handler;
mod save;

#[tokio::main]
async fn main() -> Result<()> {
    // better error reporting
    color_eyre::install()?;

    // handle authentication
    let auth = handle_auth().await?;

    // create tidal client
    let mut tidal = TidalClient::new(&auth);

    println!("logged in");
    println!("checking login..");
    println!(
        "status: {:?}",
        tidal.session.auth.check_login().await.is_ok()
    );
    println!("getting user info..");
    tidal.fetch_user_info().await?;

    println!("user info: {:#?}", tidal.user_info);

    println!("getting new arrival mixes..");
    let am = tidal.get_arrival_mixes().await?;
    for mix in am.data {
        println!("mix: {} - id: {}", mix.data_type, mix.id);
    }

    Ok(())
}
