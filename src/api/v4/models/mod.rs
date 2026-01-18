//! Data models for Cloudreve API v4
//!
//! This module is organized into submodules by functional domain:
//! - `common`: Shared types used across multiple domains
//! - `auth`: Authentication and user-related models
//! - `file`: File and directory models
//! - `user`: User management models
//! - `share`: Share link models
//! - `storage`: Storage policy and entity models
//! - `task`: Task and workflow models
//! - `site`: Site configuration models
//! - `request`: Request types
//! - `response`: Response types
//! - `dav`: WebDAV account models

// Submodules
pub mod auth;
pub mod common;
pub mod dav;
pub mod file;
pub mod request;
pub mod response;
pub mod share;
pub mod site;
pub mod storage;
pub mod task;
pub mod user;

// Re-export all types for backward compatibility and convenience
pub use auth::*;
pub use common::*;
pub use dav::*;
pub use file::*;
pub use request::*;
pub use response::*;
pub use share::*;
pub use site::*;
pub use storage::*;
pub use task::*;
pub use user::*;
