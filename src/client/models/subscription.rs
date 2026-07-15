#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSubscriptionResponse {
    pub start_date: String,
    pub valid_until: String,
    pub status: String,
    pub subscription: SubscriptionPlan,
    pub highest_sound_quality: String,
    pub premium_access: bool,
    pub can_get_trial: bool,
    pub payment_type: String,
    pub payment_overdue: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionPlan {
    #[serde(rename = "type")]
    pub subscription_type: String,
    pub offline_grace_period: u32,
}
