use crate::{credentials::TidalCredentials, session::TidalSession};

#[derive(Debug, Clone)]
pub struct Tidal {
    pub session: TidalSession,
}

impl Tidal {
    pub fn new(credentials: &TidalCredentials) -> Tidal {
        Tidal {
            session: TidalSession::new(credentials),
        }
    }
}
