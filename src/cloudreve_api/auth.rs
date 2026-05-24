//! Authentication and token management for CloudreveAPI

use crate::Error;
use crate::api::v3::models as v3_models;
use crate::api::v4::models as v4_models;
use crate::client::UnifiedClient;
use log::debug;

/// Authentication methods for CloudreveAPI
impl super::CloudreveAPI {
    /// Login with email and password
    ///
    /// This method handles both v3 (session cookie) and v4 (JWT token) authentication.
    /// After successful login, the authentication is stored internally.
    pub async fn login(&mut self, email: &str, password: &str) -> Result<LoginResponse, Error> {
        debug!("Attempting login for {}", email);

        match &mut self.inner {
            UnifiedClient::V3(client) => {
                let request = v3_models::LoginRequest {
                    user_name: email,
                    password,
                    captcha_code: None,
                };
                client.clear_session_cookie();
                let user = client.login(&request).await?;
                debug!("V3 login successful for user: {}", user.nickname);
                Ok(LoginResponse::V3(V3LoginResponse { user }))
            }
            UnifiedClient::V4(client) => {
                let request = v4_models::LoginRequest {
                    email,
                    password,
                    captcha: None,
                    ticket: None,
                };
                let login_data = client.login(&request).await?;

                // Store token internally
                client.set_token(login_data.token.access_token.clone());

                debug!("V4 login successful for user: {}", login_data.user.nickname);
                Ok(LoginResponse::V4(V4LoginResponse {
                    user: login_data.user,
                    token: login_data.token,
                }))
            }
        }
    }

    /// Login with OTP (Two-Factor Authentication)
    ///
    /// This method is available for both V3 and V4 API when the initial login returns code 203.
    /// After successful 2FA login, the authentication is stored internally.
    pub async fn login_2fa(&mut self, otp_code: &str) -> Result<LoginResponse, Error> {
        debug!("Attempting 2FA login with OTP code");

        match &mut self.inner {
            UnifiedClient::V3(client) => {
                let request = v3_models::OtpLoginRequest {
                    code: otp_code.to_string(),
                };
                let user = client.login_2fa(&request).await?;
                debug!("V3 2FA login successful for user: {}", user.nickname);
                Ok(LoginResponse::V3(V3LoginResponse { user }))
            }
            UnifiedClient::V4(client) => {
                let session_id = self.v4_session_id.as_ref().ok_or_else(|| {
                    Error::Auth("No session ID available. Please login first.".to_string())
                })?;

                let request = v4_models::TwoFactorLoginRequest {
                    otp: otp_code,
                    session_id,
                };

                debug!(
                    "Sending V4 2FA request: otp={}, session_id={}",
                    otp_code, session_id
                );

                let login_data = client.finish_2fa_login(&request).await?;

                // Store token internally
                client.set_token(login_data.token.access_token.clone());

                debug!(
                    "V4 2FA login successful for user: {}",
                    login_data.user.nickname
                );

                // Clear stored 2FA session ID
                self.v4_session_id = None;

                Ok(LoginResponse::V4(V4LoginResponse {
                    user: login_data.user,
                    token: login_data.token,
                }))
            }
        }
    }

    /// Get the current authentication token for caching purposes
    ///
    /// Returns the token info if authenticated, suitable for saving to CLI cache.
    pub fn get_token(&self) -> Result<TokenInfo, Error> {
        match &self.inner {
            UnifiedClient::V3(client) => {
                if let Some(cookie) = &client.session_cookie {
                    Ok(TokenInfo::V3Session(cookie.clone()))
                } else {
                    Err(Error::InvalidResponse(
                        "No session cookie available".to_string(),
                    ))
                }
            }
            UnifiedClient::V4(client) => {
                if let Some(token) = &client.token {
                    Ok(TokenInfo::V4Jwt(token.clone()))
                } else {
                    Err(Error::InvalidResponse("No JWT token available".to_string()))
                }
            }
        }
    }

    /// Set authentication token from cache
    ///
    /// Use this method when restoring a previous session from cache.
    /// Do not call this after `login()` - the token is already stored internally.
    pub fn set_token(&mut self, token: &str) -> Result<(), Error> {
        debug!("Setting token from cache");

        match &mut self.inner {
            UnifiedClient::V3(client) => {
                client.set_session_cookie(token.to_string());
                Ok(())
            }
            UnifiedClient::V4(client) => {
                client.set_token(token.to_string());
                Ok(())
            }
        }
    }

    /// Get the session cookie (for V3 API)
    ///
    /// Returns the session cookie if using V3 API, None otherwise.
    pub fn get_session_cookie(&self) -> Option<String> {
        match &self.inner {
            UnifiedClient::V3(client) => client.get_session_cookie().map(|s| s.to_string()),
            UnifiedClient::V4(_) => None,
        }
    }

    /// Clear authentication (session cookie or JWT token)
    ///
    /// Use this before re-authenticating to ensure fresh state.
    pub fn clear_auth(&mut self) {
        debug!("Clearing authentication");
        match &mut self.inner {
            UnifiedClient::V3(client) => {
                client.clear_session_cookie();
            }
            UnifiedClient::V4(client) => {
                client.clear_token();
            }
        }
    }
}

/// Unified login response
///
/// Wraps both V3 and V4 login responses with a common interface.
#[derive(Debug, Clone)]
pub enum LoginResponse {
    V3(V3LoginResponse),
    V4(V4LoginResponse),
}

/// V3 login response
#[derive(Debug, Clone)]
pub struct V3LoginResponse {
    pub user: v3_models::User,
}

/// V4 login response
#[derive(Debug, Clone)]
pub struct V4LoginResponse {
    pub user: v4_models::User,
    pub token: v4_models::Token,
}

impl LoginResponse {
    /// Get user nickname (common field)
    pub fn nickname(&self) -> String {
        match self {
            LoginResponse::V3(r) => r.user.nickname.clone(),
            LoginResponse::V4(r) => r.user.nickname.clone(),
        }
    }

    /// Get user email (common field)
    pub fn email(&self) -> String {
        match self {
            LoginResponse::V3(r) => r.user.user_name.clone(),
            LoginResponse::V4(r) => r.user.email.clone(),
        }
    }

    /// Get user ID (common field)
    pub fn user_id(&self) -> &str {
        match self {
            LoginResponse::V3(r) => &r.user.id,
            LoginResponse::V4(r) => &r.user.id,
        }
    }
}

/// Token information for caching
///
/// Represents either a V3 session cookie or V4 JWT token.
#[derive(Debug, Clone)]
pub enum TokenInfo {
    V3Session(String),
    V4Jwt(String),
}

impl TokenInfo {
    /// Get the raw token string
    pub fn as_str(&self) -> &str {
        match self {
            TokenInfo::V3Session(s) => s,
            TokenInfo::V4Jwt(s) => s,
        }
    }

    /// Create from raw token string with version hint
    pub fn from_string(token: String, is_v3: bool) -> Self {
        if is_v3 {
            TokenInfo::V3Session(token)
        } else {
            TokenInfo::V4Jwt(token)
        }
    }

    /// Check if this is a V3 token
    pub fn is_v3(&self) -> bool {
        matches!(self, TokenInfo::V3Session(_))
    }

    /// Check if this is a V4 token
    pub fn is_v4(&self) -> bool {
        matches!(self, TokenInfo::V4Jwt(_))
    }
}

/// CAPTCHA response for CLI use
#[derive(Debug, Clone)]
pub struct CaptchaInfo {
    /// Base64 encoded image data
    pub image: String,
    /// CAPTCHA ticket for validation
    pub ticket: String,
}

/// Additional CAPTCHA-related methods for CloudreveAPI
impl super::CloudreveAPI {
    /// Check if CAPTCHA is required for login
    pub async fn is_captcha_required(&self) -> Result<bool, Error> {
        match &self.inner {
            UnifiedClient::V3(client) => {
                // V3 uses site config to check CAPTCHA requirement
                client.is_captcha_required().await
            }
            UnifiedClient::V4(client) => client.check_captcha_requirement().await,
        }
    }

    /// Get CAPTCHA for login
    pub async fn get_captcha(&mut self) -> Result<CaptchaInfo, Error> {
        match &mut self.inner {
            UnifiedClient::V3(client) => {
                let response = client.get_captcha().await?;
                Ok(CaptchaInfo {
                    image: response.image,
                    ticket: response.ticket,
                })
            }
            UnifiedClient::V4(client) => {
                let response = client.get_captcha().await?;
                Ok(CaptchaInfo {
                    image: response.image,
                    ticket: response.ticket,
                })
            }
        }
    }

    /// Login with CAPTCHA support
    ///
    /// This method allows providing CAPTCHA code and ticket for V3 and V4 APIs.
    ///
    /// For V3, the CAPTCHA is associated with the session. When captcha_code is None,
    /// the session is cleared before login. When captcha_code is Some, the existing
    /// session (from getting the CAPTCHA) is preserved.
    pub async fn login_with_captcha(
        &mut self,
        email: &str,
        password: &str,
        captcha_code: Option<&str>,
        captcha_ticket: Option<&str>,
    ) -> Result<LoginResponse, Error> {
        debug!("Attempting login with CAPTCHA for {}", email);

        match &mut self.inner {
            UnifiedClient::V3(client) => {
                // Only clear session if this is the first attempt without CAPTCHA
                // If CAPTCHA is provided, keep the session that was established when getting the CAPTCHA
                if captcha_code.is_none() {
                    client.clear_session_cookie();
                }

                let request = v3_models::LoginRequest {
                    user_name: email,
                    password,
                    captcha_code,
                };
                let user = client.login(&request).await?;
                debug!("V3 login successful for user: {}", user.nickname);
                Ok(LoginResponse::V3(V3LoginResponse { user }))
            }
            UnifiedClient::V4(client) => {
                let request = v4_models::LoginRequest {
                    email,
                    password,
                    captcha: captcha_code,
                    ticket: captcha_ticket,
                };

                match client.login(&request).await {
                    Ok(login_data) => {
                        // Store token internally
                        client.set_token(login_data.token.access_token.clone());

                        debug!("V4 login successful for user: {}", login_data.user.nickname);
                        Ok(LoginResponse::V4(V4LoginResponse {
                            user: login_data.user,
                            token: login_data.token,
                        }))
                    }
                    Err(Error::TwoFactorRequired(session_id)) => {
                        // Store session ID for 2FA completion
                        self.v4_session_id = Some(session_id.clone());
                        Err(Error::TwoFactorRequired(session_id))
                    }
                    Err(e) => Err(e),
                }
            }
        }
    }
}
