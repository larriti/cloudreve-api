//! API version management module

use serde::{Deserialize, Serialize};

pub mod v3;
pub mod v4;

/// Version information for Cloudreve API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    /// API version (v3 or v4)
    pub api_version: String,
    /// Library version
    pub library_version: String,
    /// Server version (if available)
    pub server_version: String,
}

/// Supported API version
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiVersion {
    V3,
    V4,
}

impl ApiVersion {
    pub fn as_str(&self) -> &str {
        match self {
            ApiVersion::V3 => "v3",
            ApiVersion::V4 => "v4",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "v3" | "3" => Some(ApiVersion::V3),
            "v4" | "4" => Some(ApiVersion::V4),
            _ => None,
        }
    }
}
