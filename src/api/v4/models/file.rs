//! File-related models for Cloudreve API v4

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// File or folder metadata
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {
    #[serde(rename = "type")]
    pub r#type: FileType,
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub permission: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub size: i64,
    #[serde(default)]
    pub metadata: Option<Value>,
    pub path: String,
    #[serde(default)]
    pub capability: Option<String>,
    pub owned: bool,
    #[serde(default)]
    pub primary_entity: Option<String>,
}

/// File type enum
#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum FileType {
    File = 0,
    Folder = 1,
}

impl<'de> Deserialize<'de> for FileType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        match value {
            0 => Ok(FileType::File),
            1 => Ok(FileType::Folder),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid FileType value: {}",
                value
            ))),
        }
    }
}

/// File statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct FileStat {
    pub size: u64,
    pub created_at: String,
    pub updated_at: String,
    pub mime_type: String,
}

/// Directory list response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListResponse {
    pub files: Vec<File>,
    pub parent: File,
    pub pagination: PaginationResults,
    pub props: NavigatorProps,
    pub context_hint: String,
    pub mixed_type: bool,
    #[serde(default)]
    pub storage_policy: Option<super::storage::StoragePolicy>,
    pub view: Option<ExplorerView>,
}

/// Pagination metadata
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaginationResults {
    pub page: i32,
    pub page_size: i32,
    pub total_items: Option<i64>,
    pub next_token: Option<String>,
    pub is_cursor: bool,
}

/// Navigator capabilities
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NavigatorProps {
    pub capability: String,
    pub max_page_size: i32,
    pub order_by_options: Vec<String>,
    pub order_direction_options: Vec<String>,
}

/// Explorer view settings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExplorerView {
    pub page_size: Option<i32>,
    pub order: Option<String>,
    pub order_direction: Option<OrderDirection>,
    pub view: Option<ExplorerViewMode>,
    pub thumbnail: Option<bool>,
    pub gallery_width: Option<i32>,
    pub columns: Option<Vec<ListViewColumn>>,
}

/// Sort direction enum
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OrderDirection {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

/// View mode enum
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ExplorerViewMode {
    #[serde(rename = "list")]
    List,
    #[serde(rename = "grid")]
    Grid,
    #[serde(rename = "gallery")]
    Gallery,
}

/// List view column configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListViewColumn {
    pub r#type: i32,
    pub width: Option<i32>,
    pub props: Option<ColumnProps>,
}

/// Column properties
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColumnProps {
    pub metadata_key: Option<String>,
}

/// Extended file information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExtendedInfo {
    #[serde(default)]
    pub storage_policy: Option<super::storage::NewStoragePolicy>,
    pub storage_policy_inherited: bool,
    pub storage_used: i64,
    #[serde(default)]
    pub shares: Option<Vec<super::share::ShareLink>>,
    #[serde(default)]
    pub entities: Option<Vec<super::storage::NewEntity>>,
    pub permissions: Option<PermissionSetting>,
    #[serde(default)]
    pub direct_links: Option<Vec<super::storage::DirectLink>>,
}

/// Folder summary
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FolderSummary {
    pub size: i64,
    pub files: i64,
    pub folders: i64,
    pub completed: bool,
    pub calculated_at: String,
}

/// Permission settings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PermissionSetting {
    #[serde(rename = "user_explicit")]
    pub user_explicit: Value,
    #[serde(rename = "group_explicit")]
    pub group_explicit: Value,
    #[serde(rename = "same_group")]
    pub same_group: String,
    #[serde(rename = "other")]
    pub other: String,
    #[serde(rename = "anonymous")]
    pub anonymous: String,
    #[serde(rename = "everyone")]
    pub everyone: String,
}
