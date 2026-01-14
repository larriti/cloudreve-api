//! Error types for the Cloudreve API client

use reqwest::Error as ReqwestError;
use std::io;
use thiserror::Error;

/// Main error type for the Cloudreve API client
#[derive(Error, Debug)]
pub enum Error {
    /// HTTP request error
    #[error("HTTP request error: {0}")]
    Http(#[from] ReqwestError),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    /// API error response
    #[error("API error: {message} (code: {code})")]
    Api { code: i32, message: String },

    /// Authentication error
    #[error("Authentication error: {0}")]
    Auth(String),

    /// Invalid response error
    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    /// Invalid timestamp error
    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(String),
}
