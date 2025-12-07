use crate::{client::TidalClient, error::TidalError, requests::TidalRequest};

impl TidalClient {
    pub async fn logout(&mut self) -> Result<(), TidalError> {
        let url = "/logout".to_string();

        let mut req = TidalRequest::new(reqwest::Method::POST, url.clone());
        req.access_token = self.session.auth.access_token.clone();

        let resp = self.rq.request(req).await?;
        let status = resp.status();

        if status != reqwest::StatusCode::NO_CONTENT {
            return Err(TidalError::Logout(status.as_str().to_string()));
        }

        Ok(())
    }
}
