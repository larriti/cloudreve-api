//! Session and authentication management for Cloudreve API v3

use crate::Error;
use crate::api::v3::ApiV3Client;
use crate::api::v3::models::*;
use log::debug;

impl ApiV3Client {
    /// Login with email and password
    pub async fn login(&mut self, request: &LoginRequest<'_>) -> Result<User, Error> {
        let url = self.get_url("/user/session");
        let mut http_request = self.http_client.post(&url).json(request);

        if let Some(cookie) = &self.session_cookie {
            http_request = http_request.header("Cookie", format!("cloudreve-session={}", cookie));
        }

        let response = http_request.send().await?;

        // Extract session cookie from Set-Cookie headers
        // V3 uses Set-Cookie headers to set the session cookie
        let cookie_headers = response.headers().get_all("Set-Cookie");
        for cookie_header in cookie_headers {
            if let Ok(cookie_str) = cookie_header.to_str() {
                // Check if this Set-Cookie header contains cloudreve-session
                if cookie_str.contains("cloudreve-session=") {
                    // Parse the cookie value
                    // Format: "cloudreve-session=VALUE; Path=/; HttpOnly" etc.
                    for part in cookie_str.split(';') {
                        let part = part.trim();
                        if part.starts_with("cloudreve-session=") {
                            let session_value = part.trim_start_matches("cloudreve-session=");
                            self.session_cookie = Some(session_value.to_string());
                            debug!(
                                "Extracted V3 session cookie: {}...",
                                &session_value[..session_value.len().min(20)]
                            );
                            break;
                        }
                    }
                }
            }
        }

        debug!("V3 session_cookie after login: {:?}", self.session_cookie);

        let _status = response.status();
        let api_response: ApiResponse<User> = response.json().await?;

        match api_response.data {
            Some(user) => Ok(user),
            None => Err(Error::Api {
                code: api_response.code,
                message: api_response.msg,
            }),
        }
    }

    /// Login with OTP (Two-Factor Authentication)
    pub async fn login_2fa(&mut self, request: &OtpLoginRequest) -> Result<User, Error> {
        let url = self.get_url("/user/2fa");
        let mut http_request = self.http_client.post(&url).json(request);

        if let Some(cookie) = &self.session_cookie {
            http_request = http_request.header("Cookie", format!("cloudreve-session={}", cookie));
        }

        let response = http_request.send().await?;

        // Extract session cookie from Set-Cookie headers
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
                            "Extracted V3 session cookie (2FA): {}...",
                            &session_value[..session_value.len().min(20)]
                        );
                        break;
                    }
                }
            }
        }

        let _status = response.status();
        let api_response: ApiResponse<User> = response.json().await?;

        match api_response.data {
            Some(user) => Ok(user),
            None => Err(Error::Api {
                code: api_response.code,
                message: api_response.msg,
            }),
        }
    }

    /// Logout from current session
    pub async fn logout(&self) -> Result<(), Error> {
        let response: ApiResponse<()> = self.delete("/user/session").await?;
        if response.code == 0 {
            Ok(())
        } else {
            Err(Error::Api {
                code: response.code,
                message: response.msg,
            })
        }
    }
}
