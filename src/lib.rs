//! A Rust library for interacting with Cloudreve API
//!
//! This library provides asynchronous access to the Cloudreve API endpoints.
//! It handles authentication, request building, and response parsing.
//!
//! # Examples
//!
//! ## Using the unified client (recommended)
//!
//! ```no_run
//! use cloudreve_api::{CloudreveClient, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Auto-detect version
//!     let client = CloudreveClient::new("https://your-cloudreve-instance.com").await?;
//!
//!     // Get version information
//!     let version = client.get_version().await?;
//!     println!("API Version: {}", version.api_version);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Using version-specific clients
//!
//! ```no_run
//! use cloudreve_api::{ApiV3Client, ApiV4Client_};
//!
//! #[tokio::main]
//! async fn main() {
//!     // V3 client
//!     let v3_client = ApiV3Client::new("https://your-cloudreve-instance.com");
//!
//!     // V4 client
//!     let v4_client = ApiV4Client_::new("https://your-cloudreve-instance.com");
//! }
//! ```

pub mod api;
pub mod client;
pub mod error;

pub use api::v3::models::{
    ApiResponse, Aria2CreateRequest, Aria2Task, CopyObjectRequest, CreateDirectoryRequest,
    CreateFileRequest, DeleteObjectRequest, DirectoryList, DownloadUrl, FileSource,
    FileSourceRequest, LoginRequest, MoveObjectRequest, Object, OtpLoginRequest, Policy, Property,
    RenameObjectRequest, Share, ShareRequest, SiteConfig, SourceItems, StorageInfo,
    UploadFileRequest, UploadSession, User, UserGroup, WebdavAccount,
};
pub use api::v4::models::*;
// Main Cloudreve API client (V4, for backward compatibility)
pub use api::v4::ApiV4Client as CloudreveClient;
// Unified client with auto-detection (new)
pub use client::UnifiedClient;
pub use error::Error;

// Re-export version-specific clients for advanced use cases
pub use api::v3::ApiV3Client;
pub use api::v4::ApiV4Client as ApiV4Client_;

// Re-export API version types
pub use api::{ApiVersion, VersionInfo};

/// A result type alias for convenience
pub type Result<T> = std::result::Result<T, Error>;

/// Cloudreve API library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
