//! Cloudreve API - Simplified unified interface
//!
//! This module provides a simplified, version-agnostic interface to the Cloudreve API.
//! It automatically handles version detection, authentication, and request routing.

use crate::client::UnifiedClient;
use crate::api::v3::models as v3_models;
use crate::api::v4::models as v4_models;
use crate::api::ApiVersion;
use crate::Error;
use log::debug;

/// Unified Cloudreve API client
///
/// This client automatically detects the API version (v3 or v4) and routes
/// all requests to the appropriate endpoints. It handles authentication
/// differences transparently.
pub struct CloudreveAPI {
    inner: UnifiedClient,
    base_url: String,
}

impl CloudreveAPI {
    /// Create a new API client with automatic version detection
    ///
    /// This method probes the server to determine which API version it supports,
    /// preferring v4 over v3 when both are available.
    pub async fn new(base_url: &str) -> Result<Self, Error> {
        let base_url = base_url.trim_end_matches('/').to_string();
        debug!("Creating CloudreveAPI for {}", base_url);

        let inner = UnifiedClient::new(&base_url).await?;
        debug!("API version detected: {:?}", inner.api_version());

        Ok(Self { inner, base_url })
    }

    /// Create a new API client with a specific version
    ///
    /// This method creates a client for the specified API version without probing.
    /// Useful when the version is already known (e.g., from cached token).
    pub fn with_version(base_url: &str, version: ApiVersion) -> Result<Self, Error> {
        let base_url = base_url.trim_end_matches('/').to_string();
        debug!("Creating CloudreveAPI for {} with version {:?}", base_url, version);

        // Use a blocking version since we already know the version
        let inner = match version {
            ApiVersion::V3 => UnifiedClient::V3(crate::api::v3::ApiV3Client::new(&base_url)),
            ApiVersion::V4 => UnifiedClient::V4(crate::api::v4::ApiV4Client::new(&base_url)),
        };

        Ok(Self { inner, base_url })
    }

    /// Login with email and password
    ///
    /// This method handles both v3 (session cookie) and v4 (JWT token) authentication.
    /// After successful login, the authentication is stored internally.
    pub async fn login(&mut self, email: &str, password: &str) -> Result<LoginResponse, Error> {
        debug!("Attempting login for {}", email);

        match &mut self.inner {
            UnifiedClient::V3(client) => {
                let request = v3_models::LoginRequest {
                    user_name: email,
                    password: password,
                    captcha_code: "",
                };
                let user = client.login(&request).await?;
                debug!("V3 login successful for user: {}", user.nickname);
                Ok(LoginResponse::V3(V3LoginResponse { user }))
            }
            UnifiedClient::V4(client) => {
                let request = v4_models::LoginRequest {
                    email,
                    password,
                };
                let login_data = client.login(&request).await?;

                // Store token internally
                client.set_token(login_data.token.access_token.clone());

                debug!("V4 login successful for user: {}", login_data.user.nickname);
                Ok(LoginResponse::V4(V4LoginResponse {
                    user: login_data.user,
                    token: login_data.token,
                }))
            }
        }
    }

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

    /// Get the current authentication token for caching purposes
    ///
    /// Returns the token info if authenticated, suitable for saving to CLI cache.
    pub fn get_token(&self) -> Result<TokenInfo, Error> {
        match &self.inner {
            UnifiedClient::V3(client) => {
                if let Some(cookie) = &client.session_cookie {
                    Ok(TokenInfo::V3Session(cookie.clone()))
                } else {
                    Err(Error::InvalidResponse("No session cookie available".to_string()))
                }
            }
            UnifiedClient::V4(client) => {
                if let Some(token) = &client.token {
                    Ok(TokenInfo::V4Jwt(token.clone()))
                } else {
                    Err(Error::InvalidResponse("No JWT token available".to_string()))
                }
            }
        }
    }

    /// Set authentication token from cache
    ///
    /// Use this method when restoring a previous session from cache.
    /// Do not call this after `login()` - the token is already stored internally.
    pub fn set_token(&mut self, token: &str) -> Result<(), Error> {
        debug!("Setting token from cache");

        match &mut self.inner {
            UnifiedClient::V3(client) => {
                client.set_session_cookie(token.to_string());
                Ok(())
            }
            UnifiedClient::V4(client) => {
                client.set_token(token.to_string());
                Ok(())
            }
        }
    }

    /// Get the detected API version
    pub fn api_version(&self) -> ApiVersion {
        self.inner.api_version()
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
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

    /// Create a download URL for a file
    ///
    /// Returns a download URL that can be used to download the file.
    pub async fn create_download_url(&self, path: &str) -> Result<String, Error> {
        debug!("Creating download URL for: {}", path);

        match &self.inner {
            UnifiedClient::V3(client) => {
                // V3: Need to get file ID first, then get download URL
                // For now, return a placeholder - this needs proper implementation
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
                // Return the first URL
                if let Some(first_url) = response.urls.first() {
                    Ok(first_url.url.clone())
                } else {
                    Err(Error::InvalidResponse("No download URL returned".to_string()))
                }
            }
        }
    }

    /// Create a share link for a file or directory
    ///
    /// Creates a share link with optional expiration and password.
    pub async fn create_share(
        &self,
        path: &str,
        _name: Option<&str>,
        expires_in: Option<u32>,
        password: Option<&str>,
    ) -> Result<String, Error> {
        debug!("Creating share link for: {}", path);

        match &self.inner {
            UnifiedClient::V3(client) => {
                let request = v3_models::ShareRequest {
                    id: path.to_string(),
                    is_dir: path.ends_with('/'),
                    password: password.unwrap_or("").to_string(),
                    downloads: 0,
                    expire: expires_in.unwrap_or(0) as i32,
                    preview: true,
                };
                let share = client.create_share(&request).await?;
                Ok(share.key)
            }
            UnifiedClient::V4(client) => {
                let permissions = v4_models::PermissionSetting {
                    user_explicit: serde_json::json!({}),
                    group_explicit: serde_json::json!({}),
                    same_group: "read".to_string(),
                    other: "read".to_string(),
                    anonymous: "read".to_string(),
                    everyone: "read".to_string(),
                };
                let request = v4_models::CreateShareLinkRequest {
                    permissions,
                    uri: path.to_string(),
                    is_private: Some(password.is_some()),
                    share_view: None,
                    expire: expires_in,
                    price: None,
                    password: password.map(|p| p.to_string()),
                    show_readme: None,
                };
                let share = client.create_share_link(&request).await?;
                Ok(share)
            }
        }
    }

    /// Get the session cookie (for V3 API)
    ///
    /// Returns the session cookie if using V3 API, None otherwise.
    pub fn get_session_cookie(&self) -> Option<String> {
        match &self.inner {
            UnifiedClient::V3(client) => client.get_session_cookie().map(|s| s.to_string()),
            UnifiedClient::V4(_) => None,
        }
    }

    /// Get access to the underlying UnifiedClient
    ///
    /// This is a temporary method for advanced use cases where direct V3/V4 client access is needed.
    /// In the future, all operations should be available through CloudreveAPI methods.
    pub fn inner(&self) -> &UnifiedClient {
        &self.inner
    }

    /// Get mutable access to the underlying UnifiedClient
    ///
    /// This is a temporary method for advanced use cases where direct V3/V4 client access is needed.
    pub fn inner_mut(&mut self) -> &mut UnifiedClient {
        &mut self.inner
    }
}

/// Unified login response
///
/// Wraps both V3 and V4 login responses with a common interface.
#[derive(Debug, Clone)]
pub enum LoginResponse {
    V3(V3LoginResponse),
    V4(V4LoginResponse),
}

/// V3 login response
#[derive(Debug, Clone)]
pub struct V3LoginResponse {
    pub user: v3_models::User,
}

/// V4 login response
#[derive(Debug, Clone)]
pub struct V4LoginResponse {
    pub user: v4_models::User,
    pub token: v4_models::Token,
}

impl LoginResponse {
    /// Get user nickname (common field)
    pub fn nickname(&self) -> String {
        match self {
            LoginResponse::V3(r) => r.user.nickname.clone(),
            LoginResponse::V4(r) => r.user.nickname.clone(),
        }
    }

    /// Get user email (common field)
    pub fn email(&self) -> String {
        match self {
            LoginResponse::V3(r) => r.user.user_name.clone(),
            LoginResponse::V4(r) => r.user.email.clone(),
        }
    }

    /// Get user ID (common field)
    pub fn user_id(&self) -> &str {
        match self {
            LoginResponse::V3(r) => &r.user.id,
            LoginResponse::V4(r) => &r.user.id,
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
                    is_folder: obj.object_type == "folder",
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

/// Token information for caching
///
/// Represents either a V3 session cookie or V4 JWT token.
#[derive(Debug, Clone)]
pub enum TokenInfo {
    V3Session(String),
    V4Jwt(String),
}

impl TokenInfo {
    /// Get the raw token string
    pub fn as_str(&self) -> &str {
        match self {
            TokenInfo::V3Session(s) => s,
            TokenInfo::V4Jwt(s) => s,
        }
    }

    /// Create from raw token string with version hint
    pub fn from_string(token: String, is_v3: bool) -> Self {
        if is_v3 {
            TokenInfo::V3Session(token)
        } else {
            TokenInfo::V4Jwt(token)
        }
    }

    /// Check if this is a V3 token
    pub fn is_v3(&self) -> bool {
        matches!(self, TokenInfo::V3Session(_))
    }

    /// Check if this is a V4 token
    pub fn is_v4(&self) -> bool {
        matches!(self, TokenInfo::V4Jwt(_))
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
            FileInfo::V3(obj) => obj.object_type == "folder",
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
