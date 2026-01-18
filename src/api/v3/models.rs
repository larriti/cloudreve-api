//! Common data models for the Cloudreve API v3

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// API response wrapper for v3 API
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

/// User information for v3 API
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub user_name: String,
    pub nickname: String,
    #[serde(default)]
    pub status: i32,
    #[serde(default)]
    pub avatar: String,
    pub created_at: String,
    #[serde(default)]
    pub preferred_theme: String,
    #[serde(default)]
    pub anonymous: bool,
    pub group: UserGroup,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// User group information for v3 API
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserGroup {
    #[serde(default)]
    pub id: i32,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub allow_share: bool,
    #[serde(default)]
    pub allow_remote_download: bool,
    #[serde(default)]
    pub allow_archive_download: bool,
    #[serde(default)]
    pub share_download: bool,
    #[serde(default)]
    pub compress: bool,
    #[serde(default)]
    pub webdav: bool,
    #[serde(default)]
    pub source_batch: i32,
    #[serde(default)]
    pub advance_delete: bool,
    #[serde(default)]
    pub allow_web_dav_proxy: bool,
}

/// File or folder object for v3 API
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Object {
    pub id: String,
    pub name: String,
    pub path: String,
    pub thumb: bool,
    pub size: i64,
    #[serde(rename = "type")]
    pub object_type: String,
    pub date: String,
    pub create_date: String,
    pub source_enabled: bool,
}

/// Storage policy information for v3 API
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Policy {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub policy_type: String,
    #[serde(default)]
    pub max_size: i64,
    #[serde(default)]
    pub file_type: Option<Vec<String>>,
}

/// File or folder property for v3 API
#[derive(Debug, Serialize, Deserialize)]
pub struct Property {
    pub created_at: String,
    pub updated_at: String,
    pub policy: String,
    pub size: i64,
    pub child_folder_num: i32,
    pub child_file_num: i32,
    pub path: String,
    pub query_date: String,
}

/// Directory listing response for v3 API
#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryList {
    #[serde(default)]
    pub parent: String,
    #[serde(default)]
    pub objects: Vec<Object>,
    #[serde(default)]
    pub policy: Policy,
}

/// Upload session information for v3 API
#[derive(Debug, Serialize, Deserialize)]
pub struct UploadSession {
    #[serde(rename = "sessionID")]
    pub session_id: String,
    #[serde(rename = "chunkSize")]
    pub chunk_size: i64,
    pub expires: i64,
}

/// Upload file request for v3 API
#[derive(Debug, Serialize)]
pub struct UploadFileRequest<'a> {
    pub path: &'a str,
    pub size: i64,
    pub name: &'a str,
    pub policy_id: &'a str,
    pub last_modified: i64,
    pub mime_type: &'a str,
}

/// Download file response for v3 API
#[derive(Debug, Deserialize)]
pub struct DownloadUrl {
    pub url: String,
}

/// File source information for v3 API
#[derive(Debug, Deserialize)]
pub struct FileSource {
    pub url: String,
    pub name: String,
    pub parent: i64,
}

/// Storage information for v3 API
#[derive(Debug, Serialize, Deserialize)]
pub struct StorageInfo {
    pub used: i64,
    pub free: i64,
    pub total: i64,
}

/// Share link information for v3 API
#[derive(Debug, Deserialize)]
pub struct Share {
    pub key: String,
}

/// Share request for v3 API
#[derive(Debug, Serialize)]
pub struct ShareRequest {
    pub id: String,
    pub is_dir: bool,
    pub password: String,
    pub downloads: i32,
    pub expire: i32,
    pub preview: bool,
}

/// Site configuration for v3 API
#[derive(Debug, Serialize, Deserialize)]
pub struct SiteConfig {
    pub title: String,
    pub login_captcha: bool,
    pub reg_captcha: bool,
    pub forget_captcha: bool,
    pub email_active: bool,
    pub themes: String,
    pub default_theme: String,
    pub home_view_method: String,
    pub share_view_method: String,
    pub authn: bool,
    pub user: Option<User>,
    pub captcha_recaptcha_key: String,
    pub captcha_type: String,
    pub tcaptcha_captcha_app_id: String,
    pub register_enabled: bool,
    pub app_promotion: bool,
    pub wopi_exts: Option<Value>,
}

/// Login request for v3 API
#[derive(Debug, Serialize)]
pub struct LoginRequest<'a> {
    #[serde(rename = "userName")]
    pub user_name: &'a str,
    #[serde(rename = "Password")]
    pub password: &'a str,
    #[serde(rename = "captchaCode")]
    pub captcha_code: &'a str,
}

/// OTP login request for v3 API
#[derive(Debug, Serialize)]
pub struct OtpLoginRequest {
    pub code: String,
}

/// Create directory request for v3 API
#[derive(Debug, Serialize)]
pub struct CreateDirectoryRequest<'a> {
    pub path: &'a str,
}

/// Create file request for v3 API
#[derive(Debug, Serialize)]
pub struct CreateFileRequest<'a> {
    pub path: &'a str,
}

/// File source request for v3 API
#[derive(Debug, Serialize)]
pub struct FileSourceRequest {
    pub items: Vec<String>,
}

/// Rename object request for v3 API
#[derive(Debug, Serialize)]
pub struct RenameObjectRequest<'a> {
    pub action: &'a str,
    pub src: SourceItems<'a>,
    pub new_name: &'a str,
}

/// Source items for object operations
#[derive(Debug, Serialize)]
pub struct SourceItems<'a> {
    pub dirs: Vec<&'a str>,
    pub items: Vec<&'a str>,
}

/// Move object request for v3 API
#[derive(Debug, Serialize)]
pub struct MoveObjectRequest<'a> {
    pub action: &'a str,
    pub src_dir: &'a str,
    pub src: SourceItems<'a>,
    pub dst: &'a str,
}

/// Copy object request for v3 API
#[derive(Debug, Serialize)]
pub struct CopyObjectRequest<'a> {
    pub src_dir: &'a str,
    pub src: SourceItems<'a>,
    pub dst: &'a str,
}

/// Delete object request for v3 API
#[derive(Debug, Serialize)]
pub struct DeleteObjectRequest<'a> {
    pub items: Vec<&'a str>,
    pub dirs: Vec<&'a str>,
    pub force: bool,
    pub unlink: bool,
}

/// Object property request for v3 API
pub struct ObjectPropertyRequest<'a> {
    pub id: &'a str,
    pub is_folder: Option<bool>,
    pub trace_root: Option<bool>,
}

/// Aria2 task information for v3 API
#[derive(Debug, Deserialize)]
pub struct Aria2Task {
    pub id: String,
    pub url: String,
    pub status: String,
    pub progress: f64,
    pub created_at: String,
}

/// Aria2 create download request for v3 API
#[derive(Debug, Serialize)]
pub struct Aria2CreateRequest<'a> {
    pub dst: &'a str,
    pub url: Vec<&'a str>,
}

/// WebDAV account information for v3 API
#[derive(Debug, Deserialize)]
pub struct WebdavAccount {
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Root")]
    pub uri: String,
    #[serde(rename = "Password")]
    pub password: String,
    #[serde(rename = "CreatedAt")]
    pub created_at: String,
}
