use std::collections::HashMap;

use crate::{client::tidal::TidalClient, error::TidalError, requests::TidalRequest};

impl TidalClient {
    pub async fn subscription(&mut self) -> Result<SubscriptionInfo, TidalError> {
        let url = format!(
            "/users/{}/subscription",
            self.user_info.as_ref().unwrap().user_id
        );

        let mut req = TidalRequest::new(reqwest::Method::GET, url.clone());
        let mut params = HashMap::new();
        params.insert(
            "countryCode".to_string(),
            self.user_info.as_ref().unwrap().country_code.clone(),
        );
        req.params = Some(params);
        req.access_token = self.session.auth.access_token.clone();

        let resp = self.rq.request(req).await?;
        let body = resp.text().await?;

        let json: SubscriptionInfo = serde_json::from_str(&body)?;

        Ok(json)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionInfo {
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "validUntil")]
    pub valid_until: String,
    pub status: String,
    pub subscription: SubscriptionDetails,
    #[serde(rename = "highestSoundQuality")]
    pub highest_sound_quality: String,
    #[serde(rename = "premiumAccess")]
    pub premium_access: bool,
    #[serde(rename = "canGetTrial")]
    pub can_get_trial: bool,
    #[serde(rename = "paymentType")]
    pub payment_type: String,
    #[serde(rename = "paymentOverdue")]
    pub payment_overdue: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionDetails {
    #[serde(rename = "type")]
    pub subscription_type: String,
    #[serde(rename = "offlineGracePeriod")]
    pub offline_grace_period: u32,
}
