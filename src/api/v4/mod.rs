//! API v4 implementation

use crate::Error;
use log::debug;
use serde::Serialize;

/// WebDAV account management methods for v4 API
pub mod dav;
/// File management methods for v4 API
pub mod file;
/// Common data models for v4 API
pub mod models;
/// Session management methods for v4 API
pub mod session;
/// Share management methods for v4 API
pub mod share;
/// Site-related methods for v4 API
pub mod site;
/// URI handling utilities for v4 API
pub mod uri;
/// User management methods for v4 API
pub mod user;
/// Workflow management methods for v4 API
pub mod workflow;

/// API v4 client structure
#[derive(Debug, Clone)]
pub struct ApiV4Client {
    /// Base URL for the Cloudreve instance
    pub base_url: String,
    /// HTTP client for making requests
    pub http_client: reqwest::Client,
    /// Authentication token
    pub token: Option<String>,
}

impl ApiV4Client {
    /// Creates a new API v4 client
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            http_client: reqwest::Client::new(),
            token: None,
        }
    }

    /// Sets the authentication token
    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    /// Gets the full URL for an endpoint with /api/v4 prefix
    fn get_url(&self, endpoint: &str) -> String {
        format!(
            "{}/api/v4/{}",
            self.base_url.trim_end_matches('/'),
            endpoint.trim_start_matches('/')
        )
    }

    /// Makes a GET request to the API
    pub async fn get<T>(&self, endpoint: &str) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        let url = self.get_url(endpoint);
        let mut request = self.http_client.get(&url);

        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }
        debug!("GET URL: {}", url);

        let response = request.send().await?;
        let status = response.status();

        // Check for error status codes first
        if !status.is_success() {
            let raw_text = response.text().await?;
            // Try to parse as API error response
            if let Ok(api_response) = serde_json::from_str::<crate::ApiResponse<serde_json::Value>>(&raw_text) {
                if api_response.code != 0 {
                    return Err(Error::Api {
                        code: api_response.code,
                        message: api_response.msg,
                    });
                }
            }
            // If not a standard API response, return error with status code
            return Err(Error::Api {
                code: status.as_u16() as i32,
                message: raw_text.trim().to_string(),
            });
        }

        // Get raw response text for better error reporting
        let raw_text = response.text().await?;

        match serde_json::from_str::<T>(&raw_text) {
            Ok(json) => {
                debug!("Response status: {}, JSON: {:?}", status, json);
                Ok(json)
            }
            Err(e) => {
                debug!("JSON parse error: {}, raw response: {}", e, raw_text);
                Err(Error::Json(e))
            }
        }
    }

    /// Makes a POST request to the API
    pub async fn post<T>(&self, endpoint: &str, body: &impl Serialize) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        let url = self.get_url(endpoint);
        let mut request = self.http_client.post(&url).json(body);

        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }

        debug!("POST URL: {}", url);

        let response = request.send().await?;
        let status = response.status();

        // Check for error status codes first
        if !status.is_success() {
            let raw_text = response.text().await?;
            if let Ok(api_response) = serde_json::from_str::<crate::ApiResponse<serde_json::Value>>(&raw_text) {
                if api_response.code != 0 {
                    return Err(Error::Api {
                        code: api_response.code,
                        message: api_response.msg,
                    });
                }
            }
            return Err(Error::Api {
                code: status.as_u16() as i32,
                message: raw_text.trim().to_string(),
            });
        }

        let raw_text = response.text().await?;

        match serde_json::from_str::<T>(&raw_text) {
            Ok(json) => {
                debug!("Response status: {}, JSON: {:?}", status, json);
                Ok(json)
            }
            Err(e) => {
                debug!("JSON parse error: {}, raw response: {}", e, raw_text);
                Err(Error::Json(e))
            }
        }
    }

    /// Makes a PUT request to the API
    pub async fn put<T>(&self, endpoint: &str, body: &impl Serialize) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        let url = self.get_url(endpoint);
        let mut request = self.http_client.put(&url).json(body);

        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }
        debug!("PUT URL: {}", url);

        let response = request.send().await?;
        let status = response.status();

        // Check for error status codes first
        if !status.is_success() {
            let raw_text = response.text().await?;
            if let Ok(api_response) = serde_json::from_str::<crate::ApiResponse<serde_json::Value>>(&raw_text) {
                if api_response.code != 0 {
                    return Err(Error::Api {
                        code: api_response.code,
                        message: api_response.msg,
                    });
                }
            }
            return Err(Error::Api {
                code: status.as_u16() as i32,
                message: raw_text.trim().to_string(),
            });
        }

        let raw_text = response.text().await?;

        match serde_json::from_str::<T>(&raw_text) {
            Ok(json) => {
                debug!("Response status: {}, JSON: {:?}", status, json);
                Ok(json)
            }
            Err(e) => {
                debug!("JSON parse error: {}, raw response: {}", e, raw_text);
                Err(Error::Json(e))
            }
        }
    }

    pub async fn patch<T>(&self, endpoint: &str, body: &impl Serialize) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        let url = self.get_url(endpoint);
        let mut request = self.http_client.patch(&url).json(body);

        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }
        debug!("PATCH URL: {}", url);

        let response = request.send().await?;
        let status = response.status();

        // Check for error status codes first
        if !status.is_success() {
            let raw_text = response.text().await?;
            if let Ok(api_response) = serde_json::from_str::<crate::ApiResponse<serde_json::Value>>(&raw_text) {
                if api_response.code != 0 {
                    return Err(Error::Api {
                        code: api_response.code,
                        message: api_response.msg,
                    });
                }
            }
            return Err(Error::Api {
                code: status.as_u16() as i32,
                message: raw_text.trim().to_string(),
            });
        }

        let raw_text = response.text().await?;

        match serde_json::from_str::<T>(&raw_text) {
            Ok(json) => {
                debug!("Response status: {}, JSON: {:?}", status, json);
                Ok(json)
            }
            Err(e) => {
                debug!("JSON parse error: {}, raw response: {}", e, raw_text);
                Err(Error::Json(e))
            }
        }
    }

    /// Makes a DELETE request to the API
    pub async fn delete<T>(&self, endpoint: &str) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        let url = self.get_url(endpoint);
        let mut request = self.http_client.delete(&url);

        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }
        debug!("DELETE URL: {}", url);

        let response = request.send().await?;
        let status = response.status();

        // Check for error status codes first
        if !status.is_success() {
            let raw_text = response.text().await?;
            if let Ok(api_response) = serde_json::from_str::<crate::ApiResponse<serde_json::Value>>(&raw_text) {
                if api_response.code != 0 {
                    return Err(Error::Api {
                        code: api_response.code,
                        message: api_response.msg,
                    });
                }
            }
            return Err(Error::Api {
                code: status.as_u16() as i32,
                message: raw_text.trim().to_string(),
            });
        }

        let raw_text = response.text().await?;

        match serde_json::from_str::<T>(&raw_text) {
            Ok(json) => {
                debug!("Response status: {}, JSON: {:?}", status, json);
                Ok(json)
            }
            Err(e) => {
                debug!("JSON parse error: {}, raw response: {}", e, raw_text);
                Err(Error::Json(e))
            }
        }
    }
}
