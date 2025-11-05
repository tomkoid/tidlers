use crate::{auth::init::TidalAuth, page::TidalPage, requests, session::TidalSession};

#[derive(Debug, Clone)]
pub struct TidalClient {
    pub session: TidalSession,
    pub page: TidalPage,
}

#[derive(thiserror::Error, Debug)]
pub enum TidalError {
    #[error("not authenticated, either session and/or page doesn't have access token set")]
    NotAuthenticated,

    #[error("request error")]
    RequestError(#[from] requests::RequestClientError),
}

impl TidalClient {
    pub fn new(credentials: &TidalAuth) -> TidalClient {
        let session = TidalSession::new(credentials);
        TidalClient {
            session,
            page: TidalPage::new(),
        }
    }

    fn check_auth(&self) -> Result<bool, TidalError> {
        if self.session.auth.access_token.is_none() || !self.page.is_access_token_set() {
            Err(TidalError::NotAuthenticated)
        } else {
            Ok(true)
        }
    }

    pub async fn home(&self) -> Result<(), TidalError> {
        self.check_auth()?;

        println!("home");
        self.page.r_get("pages/home");
        Ok(())
        // Ok(self.page.get("pages/home").await?)
    }
}
