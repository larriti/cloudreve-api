//! Site-related API endpoints for Cloudreve API v3

use crate::Error;
use crate::VERSION;
use crate::api::VersionInfo;
use crate::api::v3::ApiV3Client;
use crate::api::v3::models::*;
use log::debug;

impl ApiV3Client {
    /// Get site configuration
    pub async fn get_site_config(&self) -> Result<SiteConfig, Error> {
        let response: ApiResponse<SiteConfig> = self.get("/site/config").await?;
        match response.data {
            Some(config) => Ok(config),
            None => Err(Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }

    /// Get CAPTCHA for login
    pub async fn get_captcha(&mut self) -> Result<CaptchaResponse, Error> {
        let url = self.get_url("/site/captcha");
        let mut request = self.http_client.get(&url);

        // Send existing session cookie if available
        if let Some(cookie) = &self.session_cookie {
            request = request.header("Cookie", format!("cloudreve-session={}", cookie));
            debug!(
                "Sending cookie with CAPTCHA request: {}...",
                &cookie[..cookie.len().min(20)]
            );
        }

        debug!("GET URL: {}", url);

        let response = request.send().await?;

        // Extract session cookie from Set-Cookie headers BEFORE consuming the response
        let cookie_headers = response.headers().get_all("Set-Cookie");
        for cookie_header in cookie_headers {
            if let Ok(cookie_str) = cookie_header.to_str()
                && cookie_str.contains("cloudreve-session=")
            {
                for part in cookie_str.split(';') {
                    let part = part.trim();
                    if part.starts_with("cloudreve-session=") {
                        let session_value = part.trim_start_matches("cloudreve-session=");
                        self.session_cookie = Some(session_value.to_string());
                        debug!(
                            "Updated session cookie from CAPTCHA request: {}...",
                            &session_value[..session_value.len().min(20)]
                        );
                        break;
                    }
                }
            }
        }

        let raw_text = response.text().await?;

        // Print first 200 chars for debugging
        let preview = if raw_text.len() > 200 {
            format!("{}...", &raw_text[..200])
        } else {
            raw_text.clone()
        };
        debug!("CAPTCHA response preview: {}", preview);

        // V3 API returns: { "code": 0, "data": "data:image/png;base64,...", "msg": "" }
        let api_response: ApiResponse<String> = serde_json::from_str(&raw_text).map_err(|e| {
            debug!("Failed to parse CAPTCHA response: {}", e);
            Error::Json(e)
        })?;

        match api_response.data {
            Some(image_data) => Ok(CaptchaResponse {
                image: image_data,
                ticket: String::new(),
            }),
            None => Err(Error::Api {
                code: api_response.code,
                message: api_response.msg,
            }),
        }
    }

    /// Check if CAPTCHA is required for login
    pub async fn is_captcha_required(&self) -> Result<bool, Error> {
        let config = self.get_site_config().await?;
        Ok(config.login_captcha)
    }

    /// Get user storage information
    pub async fn get_user_storage(&self) -> Result<StorageInfo, Error> {
        let response: ApiResponse<StorageInfo> = self.get("/user/storage").await?;
        match response.data {
            Some(storage) => Ok(storage),
            None => Err(Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }

    /// Ping the server and get server version
    pub async fn ping(&self) -> Result<String, Error> {
        let response: ApiResponse<String> = self.get("/site/ping").await?;
        match response.data {
            Some(version) => Ok(version),
            None => Err(Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }

    /// Get API version information
    pub async fn get_version(&self) -> Result<VersionInfo, Error> {
        let server_version = self.ping().await.unwrap_or_else(|_| "unknown".to_string());
        Ok(VersionInfo {
            api_version: "v3".to_string(),
            library_version: VERSION.to_string(),
            server_version,
        })
    }

    /// Get user settings
    pub async fn get_user_settings(&self) -> Result<StorageInfo, Error> {
        let response: ApiResponse<StorageInfo> = self.get("/user/setting").await?;
        match response.data {
            Some(settings) => Ok(settings),
            None => Err(Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }

    /// Get task queue
    pub async fn get_task_queue(&self) -> Result<Vec<Aria2Task>, Error> {
        let response: ApiResponse<Vec<Aria2Task>> = self.get("/user/setting/tasks").await?;
        match response.data {
            Some(tasks) => Ok(tasks),
            None => Err(Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }
}
