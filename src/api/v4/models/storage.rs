//! Storage policy and entity models for Cloudreve API v4

use serde::{Deserialize, Serialize};

/// Storage policy information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoragePolicy {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub max_size: u64,
    #[serde(default)]
    pub allowed_suffix: Option<Vec<String>>,
    #[serde(default)]
    pub denied_suffix: Option<Vec<String>>,
    #[serde(default)]
    pub allowed_name_regexp: Option<String>,
    #[serde(default)]
    pub denied_name_regexp: Option<String>,
    #[serde(default)]
    pub relay: Option<bool>,
    #[serde(default)]
    pub weight: Option<u64>,
    #[serde(default)]
    pub children: Option<Vec<StoragePolicy>>,
    #[serde(default)]
    pub chunk_concurrency: Option<u32>,
}

/// Extended storage policy
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewStoragePolicy {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub policy_type: StoragePolicyType,
    pub allowed_suffix: Option<Vec<String>>,
    pub denied_suffix: Option<Vec<String>>,
    pub allowed_name_regexp: Option<String>,
    pub denied_name_regexp: Option<String>,
    pub max_size: f64,
    pub relay: Option<bool>,
    pub weight: Option<f64>,
    pub children: Option<Vec<NewStoragePolicy>>,
    pub chunk_concurrency: Option<i32>,
}

/// Storage policy type enum
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum StoragePolicyType {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "qiniu")]
    Qiniu,
    #[serde(rename = "upyun")]
    Upyun,
    #[serde(rename = "oss")]
    Oss,
    #[serde(rename = "cos")]
    Cos,
    #[serde(rename = "s3")]
    S3,
    #[serde(rename = "onedrive")]
    Onedrive,
    #[serde(rename = "remote")]
    Remote,
    #[serde(rename = "obs")]
    Obs,
    #[serde(rename = "load_balance")]
    LoadBalance,
    #[serde(rename = "ks3")]
    KS3,
}

/// Storage entity
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewEntity {
    pub id: String,
    pub size: i64,
    pub r#type: EntityType,
    pub created_at: String,
    pub storage_policy: Option<NewStoragePolicy>,
    pub created_by: super::auth::NewUser,
}

/// Entity type enum
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EntityType {
    #[serde(rename = "0")]
    Primary = 0,
    #[serde(rename = "1")]
    Secondary = 1,
    #[serde(rename = "2")]
    Temporary = 2,
}

/// Direct download link
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DirectLink {
    pub id: String,
    pub url: String,
    pub downloaded: f64,
    pub created_at: String,
}

/// Node information
#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: u64,
    pub name: String,
    pub created_at: String,
}
