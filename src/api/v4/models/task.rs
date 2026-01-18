//! Task and workflow models for Cloudreve API v4

use serde::{Deserialize, Serialize};

/// Basic task information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub name: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Detailed task information
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<NewNode>,
}

/// Task status enum
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

/// Task type enum
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

/// Task summary
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    pub props: serde_json::Value,
}

/// Node information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewNode {
    pub id: String,
    pub name: String,
    pub r#type: NodeType,
    pub capabilities: String,
}

/// Node type enum
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NodeType {
    #[serde(rename = "master")]
    Master,
    #[serde(rename = "slave")]
    Slave,
}

/// Task list response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskListResponse {
    pub pagination: TaskPagination,
    pub tasks: Vec<TaskResponse>,
}

/// Task pagination metadata
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskPagination {
    pub page_size: i32,
    pub next_token: Option<String>,
    pub is_cursor: bool,
}

/// Task progress information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskProgress {
    pub progress: f64,
    pub message: String,
    pub total: Option<u64>,
    pub current: Option<u64>,
}

/// Detailed task with progress
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

/// Upload progress
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Progress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

/// File activity
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Activity {
    pub id: String,
    pub content: LogEntry,
    pub created_at: String,
    pub user: Option<super::auth::NewUser>,
    pub version_id: Option<String>,
}

/// Log entry
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogEntry {
    pub r#type: String,
    pub props: serde_json::Value,
}

/// File activities response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileActivitiesResponse {
    pub activities: Vec<Activity>,
    pub pagination: ActivitiesPagination,
}

/// Activities pagination metadata
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActivitiesPagination {
    pub page: i32,
    pub page_size: i32,
    pub next_token: Option<String>,
    pub is_cursor: bool,
}

/// List tasks request
#[derive(Debug, Serialize, Default)]
pub struct ListTasksRequest<'a> {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub status: Option<&'a str>,
    pub type_: Option<&'a str>,
}

/// Create archive request
#[derive(Debug, Serialize)]
pub struct CreateArchiveRequest<'a> {
    #[serde(rename = "src")]
    pub src: Vec<&'a str>,
    #[serde(rename = "dst")]
    pub dst: &'a str,
}

/// Extract archive request
#[derive(Debug, Serialize)]
pub struct ExtractArchiveRequest<'a> {
    #[serde(rename = "src")]
    pub src: Vec<&'a str>,
    #[serde(rename = "dst")]
    pub dst: &'a str,
}
