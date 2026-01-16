//! WebDAV account models for Cloudreve API v4

use serde::{Deserialize, Serialize};

/// WebDAV account information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DavAccount {
    pub id: String,
    pub created_at: String,
    pub name: String,
    pub uri: String,
    pub password: String,
    pub options: String,
}

/// Request to create or update a WebDAV account
#[derive(Debug, Serialize)]
pub struct CreateDavAccountRequest {
    /// Root folder path (will be converted to URI format internally)
    ///
    /// Can be:
    /// - Absolute path: "/folder"
    /// - Relative path: "folder"
    /// - Already formatted URI: "cloudreve://my/folder"
    pub uri: String,
    /// Account annotation (1-255 characters)
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_sys_files: Option<bool>,
}

/// Pagination metadata for list responses
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pagination {
    pub page: i32,
    pub page_size: i32,
    pub total_items: Option<i64>,
    pub next_page_token: Option<String>,
}

/// Response for listing WebDAV accounts
#[derive(Debug, Serialize, Deserialize)]
pub struct DavAccountsResponse {
    pub accounts: Vec<DavAccount>,
    pub pagination: Pagination,
}
