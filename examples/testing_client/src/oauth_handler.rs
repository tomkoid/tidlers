use tidlers::auth::oauth::OAuthStatus;

pub fn setup_oauth_status_listener() -> tokio::sync::mpsc::UnboundedSender<OAuthStatus> {
    // create a channel to receive oauth status updates (success, pending, etc)
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<OAuthStatus>();

    // spawn a task to listen for oauth status updates
    tokio::spawn(async move {
        loop {
            if let Some(status) = rx.recv().await {
                println!("oauth status: {:?}", status);
                if status == OAuthStatus::Success {
                    println!("you have successfully authorized the application!");
                    break;
                }
            }
        }
    });

    tx
}
