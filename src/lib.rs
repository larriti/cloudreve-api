//! A Rust library for interacting with Cloudreve API
//!
//! This library provides asynchronous access to the Cloudreve API endpoints.
//! It handles authentication, request building, and response parsing.
//!
//! # Examples
//!
//! ## Using CloudreveAPI (recommended)
//!
//! ```no_run
//! use cloudreve_api::{CloudreveAPI, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Auto-detect version
//!     let mut api = CloudreveAPI::new("https://your-cloudreve-instance.com").await?;
//!
//!     // Login
//!     api.login("user@example.com", "password").await?;
//!
//!     // List files
//!     let files = api.list_files("/", None, None).await?;
//!     println!("Found {} items", files.total_count());
//!
//!     Ok(())
//! }
//! ```

pub mod api;
pub mod client;
pub mod cloudreve_api;
pub mod error;

pub use api::v3::models::{
    ApiResponse, Aria2CreateRequest, Aria2Task, CopyObjectRequest, CreateDirectoryRequest,
    CreateFileRequest, DeleteObjectRequest, DirectoryList, DownloadUrl, FileSource,
    FileSourceRequest, LoginRequest, MoveObjectRequest, Object, OtpLoginRequest, Policy, Property,
    RenameObjectRequest, Share, ShareRequest, SiteConfig, SourceItems, StorageInfo,
    UploadFileRequest, UploadSession, User, UserGroup, WebdavAccount,
};
pub use api::v4::models::*;

// Main Cloudreve API client
pub use cloudreve_api::{
    CloudreveAPI, DeleteResult, DeleteTarget, FileInfo, FileItem, FileList, FileListAll,
    LoginResponse, TokenInfo, UserInfo, V3LoginResponse, V4LoginResponse,
};

// Legacy exports for backward compatibility
pub use api::v4::ApiV4Client as CloudreveClient;
pub use client::UnifiedClient;

// Re-export version-specific clients for advanced use cases
pub use api::v3::ApiV3Client;
pub use api::v4::ApiV4Client as ApiV4Client_;

// Re-export API version types
pub use api::{ApiVersion, VersionInfo};

// Re-export error type
pub use error::Error;

/// A result type alias for convenience
pub type Result<T> = std::result::Result<T, Error>;

/// Cloudreve API library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
