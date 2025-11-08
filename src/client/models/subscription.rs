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
