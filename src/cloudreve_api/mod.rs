//! Cloudreve API - Simplified unified interface
//!
//! This module provides a simplified, version-agnostic interface to the Cloudreve API.
//! It automatically handles version detection, authentication, and request routing.
//!
//! The module is organized into submodules:
//! - `auth`: Authentication and token management
//! - `file`: File operations (list, create, delete, rename, move, copy)
//! - `share`: Share link operations
//! - `download`: Download URL operations
//! - `dav`: WebDAV account operations

use crate::client::UnifiedClient;
use crate::api::ApiVersion;
use crate::Error;
use log::debug;

// Re-export submodule types for convenience
pub use auth::{LoginResponse, TokenInfo, V3LoginResponse, V4LoginResponse};
pub use file::{DeleteTarget, FileInfo, FileItem, FileList};
pub use user::{StorageQuota, UserInfo};
pub use share::{ShareItem, ShareUpdateProps};
pub use dav::{DavAccount, DavListResponse};

// Submodules
pub mod auth;
pub mod file;
pub mod share;
pub mod download;
pub mod user;
pub mod dav;

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

    /// Get the detected API version
    pub fn api_version(&self) -> ApiVersion {
        self.inner.api_version()
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
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

    /// Get the server version
    ///
    /// Returns the Cloudreve server version by pinging the /site/ping endpoint.
    pub async fn get_server_version(&self) -> Result<String, Error> {
        match &self.inner {
            UnifiedClient::V3(client) => client.ping().await,
            UnifiedClient::V4(client) => client.ping().await,
        }
    }
}
