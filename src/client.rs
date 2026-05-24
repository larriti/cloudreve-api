//! Unified Cloudreve client with automatic version detection

use crate::Error;
use crate::api::v3::ApiV3Client;
use crate::api::v4::ApiV4Client as ApiV4ClientInner;
use crate::api::{ApiVersion, VersionInfo};
use log::debug;

/// Unified Cloudreve client that automatically handles version differences
pub enum UnifiedClient {
    V3(ApiV3Client),
    V4(ApiV4ClientInner),
}

impl UnifiedClient {
    /// Create a new client with automatic version detection
    pub async fn new(base_url: &str) -> Result<Self, Error> {
        Self::with_version(base_url, None).await
    }

    /// Create a new client with explicit version or auto-detection
    pub async fn with_version(base_url: &str, version: Option<ApiVersion>) -> Result<Self, Error> {
        let base_url = base_url.trim_end_matches('/');

        match version {
            Some(ApiVersion::V3) => {
                debug!("Creating V3 client for {}", base_url);
                Ok(UnifiedClient::V3(ApiV3Client::new(base_url)))
            }
            Some(ApiVersion::V4) => {
                debug!("Creating V4 client for {}", base_url);
                Ok(UnifiedClient::V4(ApiV4ClientInner::new(base_url)))
            }
            None => {
                // Auto-detect version
                debug!("Auto-detecting API version for {}", base_url);
                Self::detect_version(base_url).await
            }
        }
    }

    /// Detect the API version by trying endpoints
    async fn detect_version(base_url: &str) -> Result<Self, Error> {
        let base_url = base_url.trim_end_matches('/');

        // Try V4 first (newer version)
        debug!("Trying V4 endpoint...");
        let v4_client = ApiV4ClientInner::new(base_url);
        match v4_client.ping().await {
            Ok(_) => {
                debug!("V4 endpoint available, using V4 client");
                return Ok(UnifiedClient::V4(v4_client));
            }
            Err(e) => {
                debug!("V4 endpoint failed: {}", e);
            }
        }

        // Try V3
        debug!("Trying V3 endpoint...");
        let v3_client = ApiV3Client::new(base_url);
        match v3_client.ping().await {
            Ok(_) => {
                debug!("V3 endpoint available, using V3 client");
                Ok(UnifiedClient::V3(v3_client))
            }
            Err(e) => {
                debug!("V3 endpoint failed: {}", e);
                Err(Error::InvalidResponse(
                    "Could not detect API version. Neither V3 nor V4 endpoints responded."
                        .to_string(),
                ))
            }
        }
    }

    /// Get version information
    pub async fn get_version(&self) -> Result<VersionInfo, Error> {
        match self {
            UnifiedClient::V3(client) => client.get_version().await,
            UnifiedClient::V4(client) => client.get_version().await,
        }
    }

    /// Get the API version
    pub fn api_version(&self) -> ApiVersion {
        match self {
            UnifiedClient::V3(_) => ApiVersion::V3,
            UnifiedClient::V4(_) => ApiVersion::V4,
        }
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        match self {
            UnifiedClient::V3(client) => &client.base_url,
            UnifiedClient::V4(client) => &client.base_url,
        }
    }

    /// Check if the client is using V3
    pub fn is_v3(&self) -> bool {
        matches!(self, UnifiedClient::V3(_))
    }

    /// Check if the client is using V4
    pub fn is_v4(&self) -> bool {
        matches!(self, UnifiedClient::V4(_))
    }

    /// Get V3 client reference if applicable
    pub fn as_v3(&self) -> Option<&ApiV3Client> {
        match self {
            UnifiedClient::V3(client) => Some(client),
            _ => None,
        }
    }

    /// Get V4 client reference if applicable
    pub fn as_v4(&self) -> Option<&ApiV4ClientInner> {
        match self {
            UnifiedClient::V4(client) => Some(client),
            _ => None,
        }
    }

    /// Get mutable V3 client reference if applicable
    pub fn as_v3_mut(&mut self) -> Option<&mut ApiV3Client> {
        match self {
            UnifiedClient::V3(client) => Some(client),
            _ => None,
        }
    }

    /// Get mutable V4 client reference if applicable
    pub fn as_v4_mut(&mut self) -> Option<&mut ApiV4ClientInner> {
        match self {
            UnifiedClient::V4(client) => Some(client),
            _ => None,
        }
    }

    /// Upload a file to the server
    pub async fn upload_file(
        &self,
        path: &str,
        content: Vec<u8>,
        policy_id: Option<&str>,
        overwrite: bool,
    ) -> Result<(), Error> {
        match self {
            UnifiedClient::V3(client) => {
                // V3: Get policy from parent directory
                let parent_dir = if let Some(pos) = path.rfind('/') {
                    if pos == 0 { "/" } else { &path[..pos] }
                } else {
                    "/"
                };
                let file_name = path.rsplit('/').next().unwrap_or("file");

                let dir_list = client.list_directory(parent_dir).await?;
                let policy_id = policy_id.unwrap_or(&dir_list.policy.id);

                let request = crate::api::v3::models::UploadFileRequest {
                    path: parent_dir,
                    name: file_name,
                    policy_id,
                    size: content.len() as i64,
                    last_modified: 0,
                    mime_type: "",
                };
                let session = client.upload_file(&request).await?;
                client.upload_chunk(&session.session_id, 0, content).await?;

                // Ignore complete_upload errors (may not be needed)
                let _ = client.complete_upload(&session.session_id).await;
                Ok(())
            }
            UnifiedClient::V4(client) => {
                // V4: Get policy from parent directory
                let parent_dir = if let Some(pos) = path.rfind('/') {
                    if pos == 0 { "/" } else { &path[..pos] }
                } else {
                    "/"
                };

                let policy_id_str = match client
                    .list_files(&crate::api::v4::models::ListFilesRequest {
                        path: parent_dir,
                        page: Some(0),
                        page_size: Some(1),
                        ..Default::default()
                    })
                    .await
                {
                    Ok(response) => response
                        .storage_policy
                        .map(|p| p.id)
                        .unwrap_or_else(|| "default".to_string()),
                    Err(_) => "default".to_string(),
                };

                // Use entity_type="version" for overwrite
                let entity_type = if overwrite { Some("version") } else { None };
                let request = crate::api::v4::models::CreateUploadSessionRequest {
                    uri: &crate::api::v4::uri::path_to_uri(path),
                    size: content.len() as u64,
                    policy_id: &policy_id_str,
                    last_modified: None,
                    mime_type: None,
                    metadata: None,
                    entity_type,
                };
                let session = client.create_upload_session(&request).await?;
                client
                    .upload_file_chunk(&session.session_id, 0, &content)
                    .await?;
                Ok(())
            }
        }
    }
}

// Implement Clone for the unified client
impl Clone for UnifiedClient {
    fn clone(&self) -> Self {
        match self {
            UnifiedClient::V3(client) => UnifiedClient::V3(client.clone()),
            UnifiedClient::V4(client) => UnifiedClient::V4(client.clone()),
        }
    }
}

impl std::fmt::Debug for UnifiedClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnifiedClient::V3(client) => f.debug_tuple("UnifiedClient::V3").field(client).finish(),
            UnifiedClient::V4(client) => f.debug_tuple("UnifiedClient::V4").field(client).finish(),
        }
    }
}
