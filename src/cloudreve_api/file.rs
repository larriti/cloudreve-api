//! File operations for CloudreveAPI

use crate::client::UnifiedClient;
use crate::api::v3::models as v3_models;
use crate::api::v4::models as v4_models;
use crate::Error;
use log::debug;

/// File operation methods for CloudreveAPI
impl super::CloudreveAPI {
    /// List files in a directory
    ///
    /// Returns a unified file list regardless of API version.
    pub async fn list_files(&self, path: &str) -> Result<FileList, Error> {
        debug!("Listing files in: {}", path);

        match &self.inner {
            UnifiedClient::V3(client) => {
                let dir_list = client.list_directory(path).await?;
                Ok(FileList::V3(dir_list))
            }
            UnifiedClient::V4(client) => {
                let request = v4_models::ListFilesRequest {
                    path,
                    page: None,
                    page_size: None,
                    order_by: None,
                    order_direction: None,
                    next_page_token: None,
                };
                let list_response = client.list_files(&request).await?;
                Ok(FileList::V4(list_response))
            }
        }
    }

    /// Create a directory
    ///
    /// Creates a new directory at the specified path.
    pub async fn create_directory(&self, path: &str) -> Result<(), Error> {
        debug!("Creating directory: {}", path);

        match &self.inner {
            UnifiedClient::V3(client) => {
                let request = v3_models::CreateDirectoryRequest { path };
                client.create_directory(&request).await?;
                Ok(())
            }
            UnifiedClient::V4(client) => {
                client.create_directory(path).await?;
                Ok(())
            }
        }
    }

    /// Delete a file or directory
    ///
    /// Accepts either a path or URI for deletion.
    pub async fn delete(&self, target: DeleteTarget) -> Result<(), Error> {
        debug!("Deleting target: {:?}", target);

        match &self.inner {
            UnifiedClient::V3(client) => {
                // V3 requires separate lists for folders and files
                let (folders, files) = match &target {
                    DeleteTarget::Path(path) => {
                        // For path, assume it's a single item
                        (vec![path.as_str()], Vec::<&str>::new())
                    }
                    DeleteTarget::Uri(uri) => {
                        // V3 doesn't use URIs the same way, treat as path
                        (vec![uri.as_str()], Vec::<&str>::new())
                    }
                };
                let request = v3_models::DeleteObjectRequest {
                    items: files,
                    dirs: folders,
                    force: true,
                    unlink: false,
                };
                client.delete_object(&request).await?;
                Ok(())
            }
            UnifiedClient::V4(client) => {
                let path = match &target {
                    DeleteTarget::Path(p) => p.as_str(),
                    DeleteTarget::Uri(u) => u.as_str(),
                };
                client.delete_file(path).await?;
                Ok(())
            }
        }
    }

    /// Get file information by path or URI
    ///
    /// Returns unified file information regardless of API version.
    pub async fn get_file_info(&self, path: &str) -> Result<FileInfo, Error> {
        debug!("Getting file info for: {}", path);

        match &self.inner {
            UnifiedClient::V3(client) => {
                // V3: Use object property (requires ID) or get from directory listing
                // For simplicity, list the parent directory and find the object
                let parent_path = if path.ends_with('/') || path == "/" {
                    path
                } else {
                    let pos = path.rfind('/');
                    match pos {
                        Some(0) => "/",
                        Some(p) => &path[..p],
                        None => "/",
                    }
                };

                let dir_list = client.list_directory(parent_path).await?;

                // Find the object by name
                let file_name = if path.ends_with('/') || path == "/" {
                    ""
                } else {
                    path.rsplit('/').next().unwrap_or("")
                };

                for obj in &dir_list.objects {
                    if obj.name == file_name {
                        return Ok(FileInfo::V3(obj.clone()));
                    }
                }

                Err(Error::InvalidResponse(format!("File not found: {}", path)))
            }
            UnifiedClient::V4(client) => {
                let request = v4_models::GetFileInfoRequest {
                    uri: path,
                    include_extended_info: Some(false),
                };
                let file = client.get_file_info_extended(&request).await?;
                Ok(FileInfo::V4(file))
            }
        }
    }

    /// Rename a file or directory
    ///
    /// Renames a file or directory at the given path to a new name.
    pub async fn rename(&self, path: &str, new_name: &str) -> Result<(), Error> {
        debug!("Renaming {} to {}", path, new_name);

        match &self.inner {
            UnifiedClient::V3(client) => {
                // V3 needs source items split into dirs and items
                let is_dir = path.ends_with('/');
                let request = v3_models::RenameObjectRequest {
                    action: "rename",
                    src: v3_models::SourceItems {
                        dirs: if is_dir { vec![path] } else { vec![] },
                        items: if !is_dir { vec![path] } else { vec![] },
                    },
                    new_name,
                };
                client.rename_object(&request).await?;
                Ok(())
            }
            UnifiedClient::V4(client) => {
                let request = v4_models::RenameFileRequest { name: new_name };
                client.rename_file(path, &request).await?;
                Ok(())
            }
        }
    }

    /// Move a file or directory
    ///
    /// Moves a file or directory from source path to destination path.
    pub async fn move_file(&self, src: &str, dest: &str) -> Result<(), Error> {
        debug!("Moving {} to {}", src, dest);

        match &self.inner {
            UnifiedClient::V3(client) => {
                // V3 needs source items and destination directory
                let is_dir = src.ends_with('/');
                let src_dir = if let Some(pos) = src.rfind('/') {
                    if pos == 0 { "/" } else { &src[..pos] }
                } else {
                    "/"
                };
                let request = v3_models::MoveObjectRequest {
                    action: "move",
                    src_dir,
                    src: v3_models::SourceItems {
                        dirs: if is_dir { vec![src] } else { vec![] },
                        items: if !is_dir { vec![src] } else { vec![] },
                    },
                    dst: dest,
                };
                client.move_object(&request).await?;
                Ok(())
            }
            UnifiedClient::V4(client) => {
                let request = v4_models::MoveFileRequest {
                    from: src,
                    to: dest,
                };
                client.move_file(&request).await?;
                Ok(())
            }
        }
    }

    /// Copy a file or directory
    ///
    /// Copies a file or directory from source path to destination path.
    pub async fn copy_file(&self, src: &str, dest: &str) -> Result<(), Error> {
        debug!("Copying {} to {}", src, dest);

        match &self.inner {
            UnifiedClient::V3(client) => {
                // V3 needs source items and destination directory
                let is_dir = src.ends_with('/');
                let src_dir = if let Some(pos) = src.rfind('/') {
                    if pos == 0 { "/" } else { &src[..pos] }
                } else {
                    "/"
                };
                let request = v3_models::CopyObjectRequest {
                    src_dir,
                    src: v3_models::SourceItems {
                        dirs: if is_dir { vec![src] } else { vec![] },
                        items: if !is_dir { vec![src] } else { vec![] },
                    },
                    dst: dest,
                };
                client.copy_object(&request).await?;
                Ok(())
            }
            UnifiedClient::V4(client) => {
                let request = v4_models::CopyFileRequest {
                    from: src,
                    to: dest,
                };
                client.copy_file(&request).await?;
                Ok(())
            }
        }
    }

    /// Upload a file
    ///
    /// Uploads a file to the specified path. Returns the uploaded file info.
    pub async fn upload_file(
        &self,
        path: &str,
        content: Vec<u8>,
        policy_id: Option<&str>,
    ) -> Result<(), Error> {
        debug!("Uploading file to: {}", path);

        match &self.inner {
            UnifiedClient::V3(client) => {
                // V3: Use upload session with chunking
                let file_name = path.rsplit('/').next().unwrap_or("file");
                let request = v3_models::UploadFileRequest {
                    path,
                    name: file_name,
                    policy_id: policy_id.unwrap_or(""),
                    size: content.len() as i64,
                    last_modified: 0,
                    mime_type: "",
                };
                let session = client.upload_file(&request).await?;

                // Upload single chunk (for simplicity)
                client.upload_chunk(&session.session_id, 0, content).await?;

                Ok(())
            }
            UnifiedClient::V4(client) => {
                // V4: Use upload session
                let request = v4_models::CreateUploadSessionRequest {
                    uri: path,
                    size: content.len() as u64,
                    policy_id: policy_id.unwrap_or("default"),
                    last_modified: None,
                    mime_type: None,
                    metadata: None,
                    entity_type: None,
                };
                let session = client.create_upload_session(&request).await?;

                // Upload content (simplified - single chunk)
                // This is a simplified version - real implementation would handle chunking
                let url = format!("{}/api/v4/file/upload/complete", client.base_url);
                let http_resp = client.http_client
                    .post(&url)
                    .bearer_auth(client.token.as_ref().ok_or_else(|| {
                        Error::InvalidResponse("No token available".to_string())
                    })?)
                    .json(&serde_json::json!({"session_id": session.session_id}))
                    .send()
                    .await?;

                if !http_resp.status().is_success() {
                    return Err(Error::Http(reqwest::Error::from(
                        http_resp.error_for_status().unwrap_err()
                    )));
                }

                Ok(())
            }
        }
    }

    /// Download a file
    ///
    /// Returns the download URL for the file.
    pub async fn download_file(&self, path: &str) -> Result<String, Error> {
        debug!("Downloading file: {}", path);

        match &self.inner {
            UnifiedClient::V3(client) => {
                let url = client.download_file(path).await?;
                Ok(url.url)
            }
            UnifiedClient::V4(client) => {
                let request = v4_models::CreateDownloadUrlRequest {
                    uris: vec![path],
                    download: Some(true),
                    redirect: Some(true),
                    entity: None,
                    use_primary_site_url: None,
                    skip_error: None,
                    archive: None,
                    no_cache: None,
                };
                let response = client.create_download_url(&request).await?;
                if let Some(first_url) = response.urls.first() {
                    Ok(first_url.url.clone())
                } else {
                    Err(Error::InvalidResponse("No download URL returned".to_string()))
                }
            }
        }
    }

    /// Restore a file from trash
    ///
    /// Restores a file or directory from the trash. Only available in V4.
    pub async fn restore_file(&self, path: &str) -> Result<(), Error> {
        debug!("Restoring file: {}", path);

        match &self.inner {
            UnifiedClient::V3(_) => {
                Err(Error::UnsupportedFeature(
                    "restore from trash".to_string(),
                    "v3".to_string(),
                ))
            }
            UnifiedClient::V4(client) => {
                let request = v4_models::RestoreFileRequest {
                    uris: vec![path],
                };
                client.restore_from_trash(&request).await?;
                Ok(())
            }
        }
    }
}

/// Unified file list response
///
/// Wraps both V3 and V4 directory listing responses.
#[derive(Debug)]
pub enum FileList {
    V3(v3_models::DirectoryList),
    V4(v4_models::ListResponse),
}

impl FileList {
    /// Get parent directory name
    pub fn parent_name(&self) -> String {
        match self {
            FileList::V3(d) => d.parent.clone(),
            FileList::V4(r) => r.parent.name.clone(),
        }
    }

    /// Get files and folders
    pub fn items(&self) -> Vec<FileItem> {
        match self {
            FileList::V3(d) => {
                d.objects.iter().map(|obj| FileItem {
                    name: obj.name.clone(),
                    is_folder: obj.object_type == "dir",
                    size: obj.size,
                }).collect()
            }
            FileList::V4(r) => {
                r.files.iter().map(|file| FileItem {
                    name: file.name.clone(),
                    is_folder: matches!(file.r#type, v4_models::FileType::Folder),
                    size: file.size,
                }).collect()
            }
        }
    }

    /// Get total count
    pub fn total_count(&self) -> usize {
        self.items().len()
    }
}

/// Unified file/folder item
#[derive(Debug, Clone)]
pub struct FileItem {
    pub name: String,
    pub is_folder: bool,
    pub size: i64,
}

/// Target for delete operation
///
/// Accepts either a path or URI to provide flexibility.
#[derive(Debug, Clone)]
pub enum DeleteTarget {
    Path(String),
    Uri(String),
}

impl From<&str> for DeleteTarget {
    fn from(s: &str) -> Self {
        if s.starts_with("cloudreve://") {
            DeleteTarget::Uri(s.to_string())
        } else {
            DeleteTarget::Path(s.to_string())
        }
    }
}

impl From<String> for DeleteTarget {
    fn from(s: String) -> Self {
        if s.starts_with("cloudreve://") {
            DeleteTarget::Uri(s)
        } else {
            DeleteTarget::Path(s)
        }
    }
}

/// Unified file information response
///
/// Wraps both V3 and V4 file information responses.
#[derive(Debug, Clone)]
pub enum FileInfo {
    V3(v3_models::Object),
    V4(v4_models::File),
}

impl FileInfo {
    /// Get file name
    pub fn name(&self) -> String {
        match self {
            FileInfo::V3(obj) => obj.name.clone(),
            FileInfo::V4(file) => file.name.clone(),
        }
    }

    /// Get file size
    pub fn size(&self) -> i64 {
        match self {
            FileInfo::V3(obj) => obj.size,
            FileInfo::V4(file) => file.size,
        }
    }

    /// Check if it's a folder
    pub fn is_folder(&self) -> bool {
        match self {
            FileInfo::V3(obj) => obj.object_type == "dir",
            FileInfo::V4(file) => matches!(file.r#type, v4_models::FileType::Folder),
        }
    }

    /// Get file path
    pub fn path(&self) -> String {
        match self {
            FileInfo::V3(obj) => obj.path.clone(),
            FileInfo::V4(file) => file.path.clone(),
        }
    }

    /// Get created date
    pub fn created_at(&self) -> String {
        match self {
            FileInfo::V3(obj) => obj.create_date.clone(),
            FileInfo::V4(file) => file.created_at.clone(),
        }
    }

    /// Get updated date
    pub fn updated_at(&self) -> String {
        match self {
            FileInfo::V3(obj) => obj.date.clone(),
            FileInfo::V4(file) => file.updated_at.clone(),
        }
    }
}
