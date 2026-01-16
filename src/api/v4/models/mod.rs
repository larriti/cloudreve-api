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
pub mod common;
pub mod auth;
pub mod file;
pub mod user;
pub mod share;
pub mod storage;
pub mod task;
pub mod site;
pub mod request;
pub mod response;
pub mod dav;

// Re-export all types for backward compatibility and convenience
pub use common::*;
pub use auth::*;
pub use file::*;
pub use user::*;
pub use share::*;
pub use storage::*;
pub use task::*;
pub use site::*;
pub use request::*;
pub use response::*;
pub use dav::*;
