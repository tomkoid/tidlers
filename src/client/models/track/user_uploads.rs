use crate::client::models::responses::ApiLinks;

/// Response containing user uploaded tracks
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserUploadsResponse {
    pub data: Vec<serde_json::Value>,
    #[serde(default)]
    pub included: Vec<UserUploadIncludedResource>,
    #[serde(default)]
    pub links: Option<serde_json::Value>,
}

/// Represents a user upload source file resource
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserUploadIncludedResource {
    pub id: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    #[serde(default)]
    pub attributes: Option<UserUploadResourceAttributes>,
    #[serde(default)]
    pub relationships: Option<UserUploadResourceRelationships>,
}

/// Attributes of a user upload source file
#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserUploadResourceAttributes {
    #[serde(default)]
    pub md5_hash: Option<String>,
    #[serde(default)]
    pub size: Option<u64>,
    #[serde(default)]
    pub upload_link: Option<UploadLink>,
    #[serde(default)]
    pub status: Option<UploadStatus>,
}

/// Upload link information including URL and metadata
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadLink {
    pub href: String,
    pub meta: UploadMeta,
}

/// TopArtistsMetadata for upload link
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadMeta {
    pub method: String,
    pub headers: UploadHeaders,
}

/// Headers required for upload
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UploadHeaders {
    pub content_length: String,
    pub content_md5: String,
}

/// Status information for uploaded file
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadStatus {
    pub technical_file_status: String,
    pub moderation_file_status: String,
}

/// Relationships for user uploads
#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserUploadResourceRelationships {
    #[serde(default)]
    pub owners: Option<UserUploadOwnerRelationship>,
}

/// Owner relationship information
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserUploadOwnerRelationship {
    pub links: UserUploadRelationshipLinks,
}

/// Links for relationships
pub type UserUploadRelationshipLinks = ApiLinks;
