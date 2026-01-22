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
    eyre::install()?;

    let args = Args::parse();
    let mut tidal = initialize_client().await?;

    configure_client(&mut tidal, args.debug);
    complete_oauth_if_needed(&mut tidal).await?;
    finalize_authentication(&mut tidal).await?;

    execute_command(tidal, args.command).await?;

    Ok(())
}

async fn initialize_client() -> eyre::Result<TidalClient> {
    if let Some(auth) = handle_auth().await {
        Ok(TidalClient::new(&auth))
    } else {
        load_saved_session().await
    }
}

async fn load_saved_session() -> eyre::Result<TidalClient> {
    let saved_session_data = save::get_session_data()
        .ok_or_else(|| eyre::Report::msg("No saved session data found"))?;
    
    let mut client = TidalClient::from_json(&saved_session_data)?;
    
    if client.refresh_access_token(false).await? {
        println!("Token refreshed");
    } else {
        println!("Using saved session");
    }
    
    Ok(client)
}

fn configure_client(client: &mut TidalClient, debug: bool) {
    client.set_debug_mode(debug);
    client.set_time_offset("+01:00".to_string()); // TODO: auto-detect timezone
}

async fn complete_oauth_if_needed(client: &mut TidalClient) -> eyre::Result<()> {
    if !client.waiting_for_oauth_login() {
        return Ok(());
    }

    println!("Completing OAuth login...");
    auth::handle_oauth_flow(client).await?;
    println!("OAuth complete");
    
    Ok(())
}

async fn finalize_authentication(client: &mut TidalClient) -> eyre::Result<()> {
    client.refresh_user_info().await?;
    save_session_data(&client.get_json());
    println!("Logged in\n");
    
    Ok(())
}
