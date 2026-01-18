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
                    captcha_code: "",
                };
                client.clear_session_cookie();
                let user = client.login(&request).await?;
                debug!("V3 login successful for user: {}", user.nickname);
                Ok(LoginResponse::V3(V3LoginResponse { user }))
            }
            UnifiedClient::V4(client) => {
                let request = v4_models::LoginRequest { email, password };
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
