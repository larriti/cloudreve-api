//! Response types for Cloudreve API v4

use serde::Deserialize;

/// Upload session response
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
    pub storage_policy: super::storage::StoragePolicy,
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

/// Download URL response
#[derive(Debug, Deserialize)]
pub struct DownloadUrlResponse {
    pub urls: Vec<DownloadUrlItem>,
    pub expires: String,
}

/// Download URL item
#[derive(Debug, Deserialize)]
pub struct DownloadUrlItem {
    pub url: String,
    #[serde(default)]
    pub stream_saver_display_name: Option<String>,
}

/// Archive list response
#[derive(Debug, Deserialize)]
pub struct ArchiveListResponse {
    pub files: Vec<ArchiveFileItem>,
}

/// Archive file item
#[derive(Debug, Deserialize)]
pub struct ArchiveFileItem {
    pub name: String,
    pub size: u64,
    pub r#type: String,
    pub path: String,
}

/// Viewer session response
#[derive(Debug, Deserialize)]
pub struct ViewerSessionResponse {
    pub session_id: String,
}
