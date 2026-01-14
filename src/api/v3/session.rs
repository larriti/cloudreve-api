//! Session and authentication management for Cloudreve API v3

use crate::api::v3::models::*;
use crate::api::v3::ApiV3Client;
use crate::Error;

impl ApiV3Client {
    /// Login with email and password
    pub async fn login(&mut self, request: &LoginRequest<'_>) -> Result<User, Error> {
        let url = self.get_url("/user/session");
        let mut http_request = self.http_client.post(&url).json(request);

        if let Some(cookie) = &self.session_cookie {
            http_request = http_request.header("Cookie", format!("cloudreve-session={}", cookie));
        }

        let response = http_request.send().await?;

        if let Some(set_cookie) = response.headers().get("Set-Cookie") {
            if let Ok(cookie_str) = set_cookie.to_str() {
                for cookie in cookie_str.split(';').map(|s| s.trim()) {
                    if cookie.starts_with("cloudreve-session=") {
                        let session_value = cookie.trim_start_matches("cloudreve-session=");
                        self.session_cookie = Some(session_value.to_string());
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

    /// Login with OTP (Two-Factor Authentication)
    pub async fn login_2fa(&mut self, request: &OtpLoginRequest) -> Result<User, Error> {
        let url = self.get_url("/user/2fa");
        let mut http_request = self.http_client.post(&url).json(request);

        if let Some(cookie) = &self.session_cookie {
            http_request = http_request.header("Cookie", format!("cloudreve-session={}", cookie));
        }

        let response = http_request.send().await?;

        if let Some(set_cookie) = response.headers().get("Set-Cookie") {
            if let Ok(cookie_str) = set_cookie.to_str() {
                for cookie in cookie_str.split(';').map(|s| s.trim()) {
                    if cookie.starts_with("cloudreve-session=") {
                        let session_value = cookie.trim_start_matches("cloudreve-session=");
                        self.session_cookie = Some(session_value.to_string());
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
