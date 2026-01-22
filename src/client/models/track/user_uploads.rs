/// Response containing user uploaded tracks
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserUploads {
    pub data: Vec<serde_json::Value>,
    #[serde(default)]
    pub included: Vec<UserUploadResource>,
    #[serde(default)]
    pub links: Option<serde_json::Value>,
}

/// Represents a user upload source file resource
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserUploadResource {
    pub id: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    #[serde(default)]
    pub attributes: Option<UserUploadAttributes>,
    #[serde(default)]
    pub relationships: Option<UserUploadRelationships>,
}

/// Attributes of a user upload source file
#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UserUploadAttributes {
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

/// Metadata for upload link
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
pub struct UserUploadRelationships {
    #[serde(default)]
    pub owners: Option<OwnerRelationship>,
}

/// Owner relationship information
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OwnerRelationship {
    pub links: RelationshipLinks,
}

/// Links for relationships
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipLinks {
    #[serde(rename = "self")]
    pub self_link: String,
}
