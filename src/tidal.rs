use crate::{credentials::TidalCredentials, requests, session::TidalSession};

#[derive(Debug, Clone)]
pub struct Tidal {
    pub session: TidalSession,
}

#[derive(thiserror::Error, Debug)]
pub enum TidalError {
    #[error("not authenticated")]
    NotAuthenticated,

    #[error("request error")]
    RequestError(#[from] requests::RequestClientError),
}

impl Tidal {
    pub fn new(credentials: &TidalCredentials) -> Tidal {
        Tidal {
            session: TidalSession::new(credentials),
        }
    }

    fn check_auth(&self) -> Result<bool, TidalError> {
        if self.session.credentials.access_token.is_none() {
            Err(TidalError::NotAuthenticated)
        } else {
            Ok(true)
        }
    }

    pub async fn home(&self) -> Result<(), TidalError> {
        self.check_auth()?;

        println!("home");
        Ok(())
    }
}
