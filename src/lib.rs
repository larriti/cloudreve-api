//! A Rust library for interacting with Cloudreve API
//!
//! This library provides asynchronous access to the Cloudreve API endpoints.
//! It handles authentication, request building, and response parsing.
//!
//! # Examples
//!
//! ```
//! use cloudreve_api::{CloudreveClient, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let client = CloudreveClient::new("https://your-cloudreve-instance.com");
//!     // Use the client to make API calls
//!     Ok(())
//! }
//! ```

pub mod api;
pub mod error;

pub use api::v3::models::{
    ApiResponse, Aria2CreateRequest, Aria2Task, CopyObjectRequest, CreateDirectoryRequest,
    CreateFileRequest, DeleteObjectRequest, DirectoryList, DownloadUrl, FileSource,
    FileSourceRequest, LoginRequest, MoveObjectRequest, Object, OtpLoginRequest, Policy, Property,
    RenameObjectRequest, Share, ShareRequest, SiteConfig, SourceItems, StorageInfo,
    UploadFileRequest, UploadSession, User, UserGroup, WebdavAccount,
};
pub use api::v4::models::*;
pub use error::Error;

/// Main Cloudreve API client (alias for v4)
pub use api::v4::ApiV4Client as CloudreveClient;

/// Cloudreve API v3 client
pub use api::v3::ApiV3Client;

/// A result type alias for convenience
pub type Result<T> = std::result::Result<T, Error>;
