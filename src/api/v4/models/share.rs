//! Share-related models for Cloudreve API v4

use serde::{Deserialize, Serialize};

/// Share link information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShareLink {
    pub id: String,
    pub name: String,
    pub visited: i64,
    #[serde(default)]
    pub downloaded: i64,
    #[serde(default)]
    pub price: i64,
    pub unlocked: bool,
    pub source_type: ShareSourceType,
    pub owner: super::auth::NewUser,
    pub created_at: String,
    pub expired: bool,
    pub url: String,
    #[serde(default)]
    pub permission_setting: Option<super::file::PermissionSetting>,
    #[serde(rename = "is_private")]
    pub is_private: Option<bool>,
    pub password: Option<String>,
    pub source_uri: Option<String>,
    pub share_view: Option<bool>,
    pub show_readme: Option<bool>,
    pub password_protected: Option<bool>,
    pub expires: Option<String>,
    pub expired_at: Option<String>,
    #[serde(default)]
    pub download_count: u64,
}

/// Share source type enum
#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum ShareSourceType {
    File = 0,
    Folder = 1,
}

impl<'de> Deserialize<'de> for ShareSourceType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        use serde_json::Value;

        let value = Value::deserialize(deserializer)?;
        match value {
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    match i {
                        0 => Ok(ShareSourceType::File),
                        1 => Ok(ShareSourceType::Folder),
                        _ => Err(Error::custom(format!("Invalid ShareSourceType value: {}", i))),
                    }
                } else {
                    Err(Error::custom(format!("Invalid ShareSourceType number: {}", n)))
                }
            }
            Value::String(s) => match s.as_str() {
                "0" => Ok(ShareSourceType::File),
                "1" => Ok(ShareSourceType::Folder),
                _ => Err(Error::custom(format!("Invalid ShareSourceType value: {}", s))),
            },
            _ => Err(Error::custom(format!("Invalid ShareSourceType type: {:?}", value))),
        }
    }
}

/// Create share link request
#[derive(Debug, Serialize)]
pub struct CreateShareLinkRequest {
    pub permissions: super::file::PermissionSetting,
    pub uri: String,
    pub is_private: Option<bool>,
    pub share_view: Option<bool>,
    pub expire: Option<u32>,
    pub price: Option<i32>,
    pub password: Option<String>,
    pub show_readme: Option<bool>,
}

/// Edit share link request
#[derive(Debug, Serialize)]
pub struct EditShareLinkRequest {
    pub permissions: super::file::PermissionSetting,
    pub uri: String,
    pub share_view: Option<bool>,
    pub expire: Option<u32>,
    pub price: Option<i32>,
    pub show_readme: Option<bool>,
}

/// Abuse report request
#[derive(Debug, Serialize)]
pub struct AbuseReportRequest<'a> {
    pub reason: &'a str,
}
