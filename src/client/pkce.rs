use std::collections::HashMap;

use reqwest::Method;
use tracing::info;
use url::Url;

use crate::{
    TidalClient,
    auth::pkce::PkceConfig,
    requests::TidalRequest,
    responses::OAuthTokenResponse,
    urls::{OAUTH2_V1_LOCATION, PKCE_AUTH_URL},
};

impl TidalClient {
    /// Initiates the PKCE login flow by generating a code verifier and code challenge
    /// and returning the URL to redirect the user to for authentication.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use tidlers::{TidalClient, auth::TidalAuth};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let auth = TidalAuth::with_oauth();
    /// let mut client = TidalClient::new(&auth);
    /// let pkce_url = client.initiate_pkce_login()?;
    /// println!("Please visit the following URL to authenticate: {}", pkce_url);
    /// # Ok(())
    /// # }
    /// ```
    pub fn initiate_pkce_login(&mut self) -> Result<String, crate::TidalError> {
        self.check_pkce_login()?;

        self.pkce_url()
    }

    pub async fn finish_pkce_login(&mut self, redirect_url: &str) -> Result<(), crate::TidalError> {
        self.check_pkce_login()?;

        let code = self.parse_pkce_redirect(redirect_url)?;
        let auth_response = self
            .get_auth_from_oauth2(code, &self.session.auth.pkce_config)
            .await?;

        self.session.auth.apply_oauth_token_state(
            auth_response.access_token.clone(),
            auth_response.refresh_token.clone(),
            auth_response.expires_in,
            auth_response.user_id,
            Some(auth_response.client_name.clone()),
        )?;
        self.user_info = Some(auth_response.user.clone());

        info!(
            user_id = auth_response.user_id,
            "PKCE & OAuth flow completed successfully"
        );

        Ok(())
    }

    async fn get_auth_from_oauth2(
        &self,
        code: String,
        pkce_config: &PkceConfig,
    ) -> Result<OAuthTokenResponse, crate::TidalError> {
        let mut form = HashMap::new();
        form.insert("code".to_string(), code);
        form.insert("client_id".to_string(), pkce_config.client_id.clone());
        form.insert("grant_type".to_string(), "authorization_code".to_string());
        form.insert("redirect_uri".to_string(), pkce_config.redirect_uri.clone());
        form.insert("scope".to_string(), "r_usr+w_usr+w_sub".to_string());
        form.insert(
            "code_verifier".to_string(),
            pkce_config.code_verifier.clone(),
        );
        form.insert(
            "client_unique_key".to_string(),
            pkce_config.client_unique_key.clone(),
        );

        let mut req = TidalRequest::new(Method::POST, "/token".to_string());
        req.form = Some(vec![form]);
        req.send_params_as_form = true;
        req.base_url = Some(OAUTH2_V1_LOCATION.to_string());

        let res = self.rq.request(req).await?;
        Ok(res.json().await?)
    }

    fn pkce_url(&self) -> Result<String, crate::TidalError> {
        self.check_pkce_login()?;

        let pkce_config = &self.session.auth.pkce_config;

        let mut params = HashMap::new();
        params.insert("response_type".to_string(), "code".to_string());
        params.insert("redirect_uri".to_string(), pkce_config.redirect_uri.clone());
        params.insert("client_id".to_string(), pkce_config.client_id.clone());
        params.insert("lang".to_string(), "EN".to_string());
        params.insert("appMode".to_string(), "android".to_string());
        params.insert(
            "client_unique_key".to_string(),
            pkce_config.client_unique_key.clone(),
        );
        params.insert(
            "code_challenge".to_string(),
            pkce_config.code_challenge.clone(),
        );
        params.insert("code_challenge_method".to_string(), "S256".to_string());
        params.insert("restrict_signup".to_string(), "true".to_string());

        let mut url = Url::parse(PKCE_AUTH_URL)?;
        url.query_pairs_mut().extend_pairs(params.iter());

        Ok(url.to_string())
    }

    fn check_pkce_login(&self) -> Result<(), crate::TidalError> {
        match self.session.auth.pkce_login {
            true => Ok(()),
            false => Err(crate::TidalError::Other(
                "PKCE login not configured. Use TidalAuth::with_pkce() first.".to_string(),
            )),
        }
    }

    fn parse_pkce_redirect(&self, redirect_url: &str) -> Result<String, crate::TidalError> {
        let url = Url::parse(redirect_url)?;
        let code = url
            .query_pairs()
            .find(|(key, _)| key == "code")
            .map(|(_, value)| value.to_string())
            .ok_or_else(|| {
                crate::TidalError::Other(
                    "Authorization code not found in redirect URL.".to_string(),
                )
            })?;
        Ok(code)
    }
}
