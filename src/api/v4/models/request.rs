//! Request types for Cloudreve API v4

use serde::Serialize;

/// Upload file request
#[derive(Debug, Serialize)]
pub struct UploadRequest<'a> {
    pub path: &'a str,
    pub name: Option<&'a str>,
    pub overwrite: Option<bool>,
}

/// List files request
#[derive(Debug, Serialize, Default)]
pub struct ListFilesRequest<'a> {
    pub path: &'a str,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub order_by: Option<&'a str>,
    pub order_direction: Option<&'a str>,
    pub next_page_token: Option<&'a str>,
}

/// Move file request (also used for copy with copy=true)
#[derive(Debug, Serialize)]
pub struct MoveFileRequest<'a> {
    pub uris: Vec<&'a str>,
    pub dst: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy: Option<bool>,
}

/// Copy file request
#[derive(Debug, Serialize)]
pub struct CopyFileRequest<'a> {
    pub uris: Vec<&'a str>,
    pub dst: &'a str,
}

/// Rename file request
#[derive(Debug, Serialize)]
pub struct RenameFileRequest<'a> {
    pub uri: &'a str,
    pub new_name: &'a str,
}

/// Set file permission request
#[derive(Debug, Serialize)]
pub struct SetFilePermissionRequest<'a> {
    /// File path (will be converted to URI format internally)
    ///
    /// Can be:
    /// - Absolute path: "/folder/file.txt"
    /// - Relative path: "folder/file.txt"
    /// - Already formatted URI: "cloudreve://my/folder/file.txt"
    pub uri: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_explicit: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_explicit: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub same_group: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub everyone: Option<&'a str>,
}

/// Create upload session request
#[derive(Debug, Serialize)]
pub struct CreateUploadSessionRequest<'a> {
    /// Target file path (will be converted to URI format internally)
    ///
    /// Can be:
    /// - Absolute path: "/folder/file.txt"
    /// - Relative path: "folder/file.txt"
    /// - Already formatted URI: "cloudreve://my/folder/file.txt"
    pub uri: &'a str,
    /// Size of the file in bytes
    pub size: u64,
    /// ID of the storage policy to use
    pub policy_id: &'a str,
    /// Optional Unix milliseconds timestamp of when the file is last modified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<u64>,
    /// Optional mime type of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<&'a str>,
    /// Optional key-value of file metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Optional blob type. "version" overwrites existing files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<&'a str>,
}

/// Delete upload session request
#[derive(Debug, Serialize)]
pub struct DeleteUploadSessionRequest<'a> {
    /// ID of the upload session
    pub id: &'a str,
    /// Target file path (will be converted to URI format internally)
    ///
    /// Can be:
    /// - Absolute path: "/folder/file.txt"
    /// - Relative path: "folder/file.txt"
    /// - Already formatted URI: "cloudreve://my/folder/file.txt"
    pub uri: &'a str,
}

/// Move/copy file request
#[derive(Debug, Serialize)]
pub struct MoveCopyFileRequest<'a> {
    pub from: Vec<&'a str>,
    pub to: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy: Option<bool>,
}

/// Update file content request
#[derive(Debug, Serialize)]
pub struct UpdateFileContentRequest<'a> {
    /// File path (will be converted to URI format internally)
    ///
    /// Can be:
    /// - Absolute path: "/folder/file.txt"
    /// - Relative path: "folder/file.txt"
    /// - Already formatted URI: "cloudreve://my/folder/file.txt"
    pub uri: &'a str,
    pub content: &'a str,
}

/// Create viewer session request
#[derive(Debug, Serialize)]
pub struct CreateViewerSessionRequest<'a> {
    /// File path (will be converted to URI format internally)
    ///
    /// Can be:
    /// - Absolute path: "/folder/file.txt"
    /// - Relative path: "folder/file.txt"
    /// - Already formatted URI: "cloudreve://my/folder/file.txt"
    pub uri: &'a str,
}

/// Create file request
#[derive(Debug, Serialize)]
pub struct CreateFileRequest<'a> {
    pub path: &'a str,
    pub name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<bool>,
}

/// Rename multiple request
#[derive(Debug, Serialize)]
pub struct RenameMultipleRequest<'a> {
    pub uris: Vec<&'a str>,
    pub names: Vec<&'a str>,
}

/// Create download URL request
#[derive(Debug, Serialize)]
pub struct CreateDownloadUrlRequest<'a> {
    /// List of file paths (will be converted to URI format internally)
    ///
    /// Each path can be:
    /// - Absolute path: "/folder/file.txt"
    /// - Relative path: "folder/file.txt"
    /// - Already formatted URI: "cloudreve://my/folder/file.txt"
    pub uris: Vec<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_primary_site_url: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_error: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_cache: Option<bool>,
}

/// Restore file request
#[derive(Debug, Serialize)]
pub struct RestoreFileRequest<'a> {
    /// List of file paths to restore (will be converted to URI format internally)
    ///
    /// Each path can be:
    /// - Absolute path: "/folder/file.txt"
    /// - Relative path: "folder/file.txt"
    /// - Already formatted URI: "cloudreve://my/folder/file.txt"
    pub uris: Vec<&'a str>,
}

/// Update metadata request
#[derive(Debug, Serialize)]
pub struct UpdateMetadataRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_metadata: Option<bool>,
}

/// Mount storage policy request
#[derive(Debug, Serialize)]
pub struct MountStoragePolicyRequest {
    pub policy_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherit_to_children: Option<bool>,
}

/// Update view request
#[derive(Debug, Serialize)]
pub struct UpdateViewRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gallery_width: Option<i32>,
}

/// Get file info request
#[derive(Debug, Serialize)]
pub struct GetFileInfoRequest<'a> {
    /// File path (will be converted to URI format internally)
    ///
    /// Can be:
    /// - Absolute path: "/folder/file.txt"
    /// - Relative path: "folder/file.txt"
    /// - Already formatted URI: "cloudreve://my/folder/file.txt"
    pub uri: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_extended_info: Option<bool>,
}

/// Get archive list request
#[derive(Debug, Serialize)]
pub struct GetArchiveListRequest<'a> {
    /// File path (will be converted to URI format internally)
    ///
    /// Can be:
    /// - Absolute path: "/folder/file.txt"
    /// - Relative path: "folder/file.txt"
    /// - Already formatted URI: "cloudreve://my/folder/file.txt"
    pub uri: &'a str,
}

/// Relocate request
#[derive(Debug, Serialize)]
pub struct RelocateRequest<'a> {
    #[serde(rename = "src")]
    pub src: Vec<&'a str>,
    #[serde(rename = "dst_policy_id")]
    pub dst_policy_id: &'a str,
}

/// Import request
#[derive(Debug, Serialize)]
pub struct ImportRequest<'a> {
    #[serde(rename = "src")]
    pub src: &'a str,
    #[serde(rename = "dst")]
    pub dst: &'a str,
    #[serde(rename = "user_id")]
    pub user_id: &'a str,
    #[serde(rename = "policy_id")]
    pub policy_id: i32,
    #[serde(rename = "extract_media_meta")]
    pub extract_media_meta: Option<bool>,
    #[serde(rename = "recursive")]
    pub recursive: Option<bool>,
}

/// Select download files request
#[derive(Debug, Serialize)]
pub struct SelectDownloadFilesRequest<'a> {
    pub selected_files: Vec<&'a str>,
}

/// Delete file request
#[derive(Debug, Serialize)]
pub struct DeleteFileRequest<'a> {
    /// List of file paths to delete (will be converted to URI format internally)
    ///
    /// Each path can be:
    /// - Absolute path: "/folder/file.txt"
    /// - Relative path: "folder/file.txt"
    /// - Already formatted URI: "cloudreve://my/folder/file.txt"
    pub uris: Vec<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlink: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_soft_delete: Option<bool>,
}

/// Create download request (alias for remote download)
#[derive(Debug, Serialize)]
pub struct CreateDownloadRequest<'a> {
    #[serde(rename = "dst")]
    pub dst: &'a str,
    #[serde(rename = "src")]
    pub src: Vec<&'a str>,
    #[serde(rename = "preferred_node_id")]
    pub preferred_node_id: Option<String>,
}
