use color_eyre::eyre::Result;
use tidlers::{auth::TidalAuth, client::TidalClient};

#[tokio::main]
async fn main() -> Result<()> {
    // better error reporting
    color_eyre::install()?;

    let auth = TidalAuth::with_pkce();
    let mut client = TidalClient::new(&auth);
    let pkce_url = client.initiate_pkce_login()?;

    println!(
        "Please visit the following URL to authenticate: {}",
        pkce_url
    );

    println!(
        "After authenticating, you will be redirected to an oops URL. Please copy the full URL and paste it here:"
    );
    let mut input_url = String::new();
    std::io::stdin().read_line(&mut input_url)?;
    let input_url = input_url.trim();

    client.finish_pkce_login(input_url).await?;

    // try to refresh the token (you dont have to do this, just demonstrating that you can force a
    // token refresh)
    client.refresh_access_token(true).await?;

    dbg!(client.get_user_info().await?);
    dbg!(client.get_json());

    Ok(())
}
