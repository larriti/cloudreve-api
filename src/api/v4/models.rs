//! Common data models for the Cloudreve API v4

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub nickname: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub avatar: Option<String>,
    pub created_at: String,
    #[serde(default)]
    pub group: Option<UserGroup>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserGroup {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub permission: Option<String>,
    #[serde(default)]
    pub direct_link_batch_size: Option<u64>,
    #[serde(default)]
    pub trash_retention: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: String,
    pub access_expires: String,
    pub refresh_expires: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginData {
    pub user: User,
    pub token: Token,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileStat {
    pub size: u64,
    pub created_at: String,
    pub updated_at: String,
    pub mime_type: String,
}

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
    pub owner: NewUser,
    pub created_at: String,
    pub expired: bool,
    pub url: String,
    #[serde(default)]
    pub permission_setting: Option<PermissionSetting>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: u64,
    pub name: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct LoginRequest<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Serialize)]
pub struct TwoFactorLoginRequest<'a> {
    pub email: &'a str,
    pub password: &'a str,
    pub code: &'a str,
    pub ticket: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct RefreshTokenRequest<'a> {
    pub refresh_token: &'a str,
}

#[derive(Debug, Serialize)]
pub struct RegisterRequest<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub email: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct UpdateProfileRequest<'a> {
    pub nickname: Option<&'a str>,
    pub email: Option<&'a str>,
    pub avatar: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct ChangePasswordRequest<'a> {
    pub old_password: &'a str,
    pub new_password: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct Quota {
    pub used: u64,
    pub total: u64,
    #[serde(default)]
    pub storage_pack_total: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSettings {
    pub theme: Option<String>,
    pub language: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UploadRequest<'a> {
    pub path: &'a str,
    pub name: Option<&'a str>,
    pub overwrite: Option<bool>,
}

#[derive(Debug, Serialize, Default)]
pub struct ListFilesRequest<'a> {
    pub path: &'a str,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub order_by: Option<&'a str>,
    pub order_direction: Option<&'a str>,
    pub next_page_token: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct MoveFileRequest<'a> {
    pub from: &'a str,
    pub to: &'a str,
}

#[derive(Debug, Serialize)]
pub struct CopyFileRequest<'a> {
    pub from: &'a str,
    pub to: &'a str,
}

#[derive(Debug, Serialize)]
pub struct RenameFileRequest<'a> {
    pub name: &'a str,
}

#[derive(Debug, Serialize)]
pub struct CreateShareLinkRequest {
    pub permissions: PermissionSetting,
    pub uri: String,
    pub is_private: Option<bool>,
    pub share_view: Option<bool>,
    pub expire: Option<u32>,
    pub price: Option<i32>,
    pub password: Option<String>,
    pub show_readme: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct EditShareLinkRequest {
    pub permissions: PermissionSetting,
    pub uri: String,
    pub share_view: Option<bool>,
    pub expire: Option<u32>,
    pub price: Option<i32>,
    pub show_readme: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct AbuseReportRequest<'a> {
    pub reason: &'a str,
}

#[derive(Debug, Serialize)]
pub struct CreateRemoteDownloadRequest<'a> {
    pub url: &'a str,
    pub path: Option<&'a str>,
    pub node_id: Option<u64>,
}

#[derive(Debug, Serialize, Default)]
pub struct ListTasksRequest<'a> {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub status: Option<&'a str>,
    pub type_: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct CreateArchiveRequest<'a> {
    pub files: Vec<&'a str>,
    pub name: &'a str,
    pub path: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct ExtractArchiveRequest<'a> {
    pub archive_uri: &'a str,
    pub path: Option<&'a str>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TaskProgress {
    pub progress: f64,
    pub message: String,
    pub total: Option<u64>,
    pub current: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DetailedTask {
    pub id: String,
    pub name: String,
    pub status: String,
    pub type_: String,
    pub created_at: String,
    pub updated_at: String,
    pub progress: Option<TaskProgress>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewUser {
    pub id: String,
    pub email: Option<String>,
    pub nickname: Option<String>,
    pub created_at: String,
    #[serde(default)]
    pub anonymous: Option<bool>,
    #[serde(default)]
    pub group: Option<NewGroup>,
    #[serde(default)]
    pub status: Option<UserStatus>,
    #[serde(default)]
    pub avatar: Option<AvatarType>,
    #[serde(default)]
    pub preferred_theme: Option<String>,
    #[serde(default)]
    pub credit: Option<i64>,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub disable_view_sync: Option<String>,
    #[serde(default)]
    pub share_links_in_profile: Option<ShareLinkVisibility>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "manual_banned")]
    ManualBanned,
    #[serde(rename = "sys_banned")]
    SysBanned,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AvatarType {
    #[serde(rename = "file")]
    File,
    #[serde(rename = "gravatar")]
    Gravatar,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ShareLinkVisibility {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "all_share")]
    AllShare,
    #[serde(rename = "hide_share")]
    HideShare,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewGroup {
    pub id: String,
    pub name: String,
    pub permission: String,
    pub direct_link_batch_size: i64,
    pub trash_retention: i64,
}

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
    pub primary_entity: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreditChangeRecord {
    pub id: String,
    pub amount: i64,
    pub reason: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentRecord {
    pub id: String,
    pub amount: f64,
    pub method: String,
    pub status: String,
    pub created_at: String,
    pub transaction_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorSetup {
    pub secret: String,
    pub qr_code: String,
    pub recovery_codes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorVerify {
    pub code: String,
}

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
            _ => Err(serde::de::Error::custom(format!("Invalid FileType value: {}", value))),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExtendedInfo {
    pub storage_policy: Option<NewStoragePolicy>,
    pub storage_policy_inherited: bool,
    pub storage_used: i64,
    pub shares: Option<Vec<ShareLink>>,
    pub entities: Option<Vec<NewEntity>>,
    pub permissions: Option<PermissionSetting>,
    pub direct_links: Option<Vec<DirectLink>>,
}

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

        // Deserialize as a JSON value first to check the type
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewEntity {
    pub id: String,
    pub size: i64,
    pub r#type: EntityType,
    pub created_at: String,
    pub storage_policy: Option<NewStoragePolicy>,
    pub created_by: NewUser,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EntityType {
    #[serde(rename = "0")]
    Primary = 0,
    #[serde(rename = "1")]
    Secondary = 1,
    #[serde(rename = "2")]
    Temporary = 2,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DirectLink {
    pub id: String,
    pub url: String,
    pub downloaded: f64,
    pub created_at: String,
}

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FolderSummary {
    pub size: i64,
    pub files: i64,
    pub folders: i64,
    pub completed: bool,
    pub calculated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListResponse {
    pub files: Vec<File>,
    pub parent: File,
    pub pagination: PaginationResults,
    pub props: NavigatorProps,
    pub context_hint: String,
    pub mixed_type: bool,
    pub storage_policy: Option<StoragePolicy>,
    pub view: Option<ExplorerView>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaginationResults {
    pub page: i32,
    pub page_size: i32,
    pub total_items: Option<i64>,
    pub next_token: Option<String>,
    pub is_cursor: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NavigatorProps {
    pub capability: String,
    pub max_page_size: i32,
    pub order_by_options: Vec<String>,
    pub order_direction_options: Vec<String>,
}

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OrderDirection {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ExplorerViewMode {
    #[serde(rename = "list")]
    List,
    #[serde(rename = "grid")]
    Grid,
    #[serde(rename = "gallery")]
    Gallery,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListViewColumn {
    pub r#type: i32,
    pub width: Option<i32>,
    pub props: Option<ColumnProps>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColumnProps {
    pub metadata_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskResponse {
    pub created_at: String,
    pub updated_at: String,
    pub id: String,
    pub status: TaskStatus,
    pub r#type: TaskType,
    pub summary: Option<TaskSummary>,
    pub duration: Option<i64>,
    pub resume_time: Option<i64>,
    pub error: Option<String>,
    pub error_history: Option<Vec<String>>,
    pub retry_count: Option<i32>,
    pub node: NewNode,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TaskStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "suspending")]
    Suspending,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "completed")]
    Completed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TaskType {
    #[serde(rename = "media_meta")]
    MediaMeta,
    #[serde(rename = "entity_recycle_routine")]
    EntityRecycleRoutine,
    #[serde(rename = "explicit_entity_recycle")]
    ExplicitEntityRecycle,
    #[serde(rename = "upload_sentinel_check")]
    UploadSentinelCheck,
    #[serde(rename = "create_archive")]
    CreateArchive,
    #[serde(rename = "extract_archive")]
    ExtractArchive,
    #[serde(rename = "relocate")]
    Relocate,
    #[serde(rename = "remote_download")]
    RemoteDownload,
    #[serde(rename = "import")]
    Import,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskSummary {
    pub phase: String,
    pub props: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewNode {
    pub id: String,
    pub name: String,
    pub r#type: NodeType,
    pub capabilities: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NodeType {
    #[serde(rename = "master")]
    Master,
    #[serde(rename = "slave")]
    Slave,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskListResponse {
    pub pagination: TaskPagination,
    pub tasks: Vec<TaskResponse>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskPagination {
    pub page_size: i32,
    pub next_token: String,
    pub is_cursor: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Progress {
    pub total: i64,
    pub current: i64,
    pub identifier: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Activity {
    pub id: String,
    pub content: LogEntry,
    pub created_at: String,
    pub user: Option<NewUser>,
    pub version_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogEntry {
    pub r#type: String,
    pub props: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileActivitiesResponse {
    pub activities: Vec<Activity>,
    pub pagination: ActivitiesPagination,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActivitiesPagination {
    pub page: i32,
    pub page_size: i32,
    pub next_token: String,
    pub is_cursor: bool,
}

#[derive(Debug, Deserialize)]
pub struct CaptchaResponse {
    pub image: String,
    pub ticket: String,
}

#[derive(Debug, Deserialize)]
pub struct SiteConfig {
    pub instance_id: Option<String>,
    pub title: Option<String>,
    pub login_captcha: Option<bool>,
    pub reg_captcha: Option<bool>,
    pub forget_captcha: Option<bool>,
    pub abuse_report_captcha: Option<bool>,
    pub themes: Option<String>,
    pub default_theme: Option<String>,
    pub authn: Option<bool>,
    pub user: Option<NewUser>,
    pub captcha_re_captcha_key: Option<String>,
    pub captcha_cap_instance_url: String,
    pub captcha_cap_site_key: String,
    pub site_notice: Option<String>,
    pub captcha_type: Option<String>,
    pub turnstile_site_id: Option<String>,
    pub register_enabled: Option<bool>,
    pub qq_enabled: Option<bool>,
    pub sso_enabled: Option<bool>,
    pub sso_display_name: Option<String>,
    pub sso_icon: Option<String>,
    pub oidc_enabled: Option<bool>,
    pub oidc_display_name: Option<String>,
    pub oidc_icon: Option<String>,
    pub logo: Option<String>,
    pub logo_light: Option<String>,
    pub tos_url: Option<String>,
    pub privacy_policy_url: Option<String>,
    pub icons: Option<String>,
    pub emoji_preset: Option<String>,
    pub point_enabled: Option<bool>,
    pub share_point_gain_rate: Option<f64>,
    pub map_provider: Option<String>,
    pub google_map_tile_type: Option<String>,
    pub file_viewers: Option<Vec<FileViewer>>,
    pub max_batch_size: Option<f64>,
    pub app_promotion: Option<bool>,
    pub app_feedback: Option<String>,
    pub app_forum: Option<String>,
    pub payment: Option<PaymentSetting>,
    pub anonymous_purchase: Option<bool>,
    pub point_price: Option<f64>,
    pub shop_nav_enabled: Option<bool>,
    pub storage_products: Option<Vec<StorageProduct>>,
    pub group_skus: Option<Vec<GroupSKU>>,
    pub thumbnail_width: Option<f64>,
    pub thumbnail_height: Option<f64>,
    pub custom_props: Option<Vec<CustomProps>>,
    pub custom_nav_items: Option<Vec<CustomNavItem>>,
    pub custom_html: Option<CustomHTML>,
    pub mapbox_ak: Option<String>,
    pub thumb_exts: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileViewer {
    pub extensions: Vec<String>,
    pub handler: String,
    pub name: String,
    pub priority: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentSetting {
    pub providers: Vec<PaymentProvider>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentProvider {
    pub id: String,
    pub name: String,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageProduct {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub storage: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupSKU {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub group_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomProps {
    pub key: String,
    pub name: String,
    pub r#type: String,
    pub options: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomNavItem {
    pub icon: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomHTML {
    pub head: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginPreparation {
    pub webauthn_enabled: bool,
    pub sso_enabled: bool,
    pub password_enabled: bool,
    pub qq_enabled: bool,
}

#[derive(Debug, Serialize)]
pub struct OpenIdPrepareRequest<'a> {
    pub hint: Option<&'a str>,
    pub linking: Option<bool>,
    pub provider: i32,
}

#[derive(Debug, Serialize)]
pub struct OpenIdFinishRequest<'a> {
    pub code: &'a str,
    pub session_id: &'a str,
    pub provider_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct PasskeySignInPreparation {
    pub session_id: String,
    pub options: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct PasskeySignInRequest<'a> {
    pub response: &'a str,
    pub session_id: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub user: NewUser,
    pub token: Token,
}

#[derive(Debug, Serialize)]
pub struct SearchUserRequest<'a> {
    pub query: &'a str,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct UpdateUserSettingRequest<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

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

#[derive(Debug, Deserialize)]
pub struct UploadSessionResponse {
    pub session_id: String,
    #[serde(default)]
    pub upload_id: Option<String>,
    pub chunk_size: u64,
    pub expires: u64,
    #[serde(default)]
    pub upload_urls: Option<Vec<String>>,
    #[serde(default)]
    pub credential: Option<String>,
    #[serde(default)]
    pub complete_url: Option<String>,
    pub storage_policy: StoragePolicy,
    #[serde(default)]
    pub mime_type: Option<String>,
    #[serde(default)]
    pub upload_policy: Option<String>,
}

impl UploadSessionResponse {
    /// Calculate total number of chunks based on file size and chunk size
    pub fn total_chunks(&self, file_size: u64) -> u32 {
        if self.chunk_size == 0 {
            return 1;
        }
        file_size.div_ceil(self.chunk_size) as u32
    }
}

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

#[derive(Debug, Serialize)]
pub struct MoveCopyFileRequest<'a> {
    pub from: Vec<&'a str>,
    pub to: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy: Option<bool>,
}

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

#[derive(Debug, Deserialize)]
pub struct ViewerSessionResponse {
    pub session_id: String,
}

#[derive(Debug, Serialize)]
pub struct CreateFileRequest<'a> {
    pub path: &'a str,
    pub name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct RenameMultipleRequest<'a> {
    pub uris: Vec<&'a str>,
    pub names: Vec<&'a str>,
}

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

#[derive(Debug, Deserialize)]
pub struct DownloadUrlResponse {
    pub urls: Vec<DownloadUrlItem>,
    pub expires: String,
}

#[derive(Debug, Deserialize)]
pub struct DownloadUrlItem {
    pub url: String,
    #[serde(default)]
    pub stream_saver_display_name: Option<String>,
}

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

#[derive(Debug, Serialize)]
pub struct UpdateMetadataRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_metadata: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct MountStoragePolicyRequest {
    pub policy_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherit_to_children: Option<bool>,
}

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

#[derive(Debug, Deserialize)]
pub struct ArchiveListResponse {
    pub files: Vec<ArchiveFileItem>,
}

#[derive(Debug, Deserialize)]
pub struct ArchiveFileItem {
    pub name: String,
    pub size: u64,
    pub r#type: String,
    pub path: String,
}

#[derive(Debug, Serialize)]
pub struct CreateDownloadRequest<'a> {
    pub url: &'a str,
    pub path: Option<&'a str>,
    pub node_id: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct SelectDownloadFilesRequest<'a> {
    pub selected_files: Vec<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct RelocateRequest<'a> {
    pub files: Vec<&'a str>,
    pub target_policy_id: &'a str,
    pub path: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct ImportRequest<'a> {
    pub source_url: &'a str,
    pub target_path: &'a str,
    pub node_id: Option<u64>,
}

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
