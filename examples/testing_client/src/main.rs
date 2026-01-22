use clap::Parser;
use tidlers::client::TidalClient;

use crate::{args::Args, auth::handle_auth, commands::execute_command, save::save_session_data};

mod args;
mod auth;
mod commands;
mod oauth_handler;
mod save;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // better error reporting
    eyre::install()?;

    // parse command line arguments
    let args = Args::parse();

    // handle authentication and create Tidal client
    let mut tidal = if let Some(auth) = handle_auth().await {
        TidalClient::new(&auth)
    } else {
        let saved_session_data = save::get_session_data().unwrap();
        let mut cl = TidalClient::from_json(&saved_session_data)?;

        let refreshed = cl.refresh_access_token(false).await?;
        if refreshed {
            println!("token refreshed from saved session data");
        } else {
            println!("using saved session data");
        }

        cl
    };

    tidal.set_debug_mode(args.debug);
    tidal.set_time_offset("+01:00".to_string()); // TODO: change this based on timezone

    // if waiting for oauth login, handle oauth flow
    if tidal.waiting_for_oauth_login() {
        println!("handling oauth flow..");
        auth::handle_oauth_flow(&mut tidal).await?;
        println!("oauth flow complete");
    }

    println!("logged in");

    // refresh user info for all commands
    tidal.refresh_user_info().await?;
    save_session_data(&tidal.get_json());

    // execute command
    execute_command(tidal, args.command).await?;

    Ok(())
}
