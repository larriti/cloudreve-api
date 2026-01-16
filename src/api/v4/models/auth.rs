//! Authentication and user-related models for Cloudreve API v4

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// User information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub nickname: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub avatar: Option<String>,
    pub created_at: String,
    #[serde(default)]
    pub group: Option<UserGroup>,
}

/// User group information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserGroup {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub permission: Option<String>,
    #[serde(default)]
    pub direct_link_batch_size: Option<u64>,
    #[serde(default)]
    pub trash_retention: Option<u64>,
}

/// JWT token information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: String,
    pub access_expires: String,
    pub refresh_expires: String,
}

/// Login response containing user and token
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginData {
    pub user: User,
    pub token: Token,
}

/// Extended user information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewUser {
    pub id: String,
    pub email: Option<String>,
    pub nickname: Option<String>,
    pub created_at: String,
    #[serde(default)]
    pub anonymous: Option<bool>,
    #[serde(default)]
    pub group: Option<NewGroup>,
    #[serde(default)]
    pub status: Option<UserStatus>,
    #[serde(default)]
    pub avatar: Option<AvatarType>,
    #[serde(default)]
    pub preferred_theme: Option<String>,
    #[serde(default)]
    pub credit: Option<i64>,
    #[serde(default)]
    pub language: Option<String>,
    #[serde(default)]
    pub disable_view_sync: Option<String>,
    #[serde(default)]
    pub share_links_in_profile: Option<ShareLinkVisibility>,
}

/// Extended group information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewGroup {
    pub id: String,
    pub name: String,
    pub permission: String,
    pub direct_link_batch_size: i64,
    pub trash_retention: i64,
}

/// User status enum
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "manual_banned")]
    ManualBanned,
    #[serde(rename = "sys_banned")]
    SysBanned,
}

/// Avatar type enum
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AvatarType {
    #[serde(rename = "file")]
    File,
    #[serde(rename = "gravatar")]
    Gravatar,
}

/// Share link visibility enum
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ShareLinkVisibility {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "all_share")]
    AllShare,
    #[serde(rename = "hide_share")]
    HideShare,
}

/// Login request
#[derive(Debug, Serialize)]
pub struct LoginRequest<'a> {
    pub email: &'a str,
    pub password: &'a str,
}

/// Two-factor login request
#[derive(Debug, Serialize)]
pub struct TwoFactorLoginRequest<'a> {
    pub email: &'a str,
    pub password: &'a str,
    pub code: &'a str,
    pub ticket: Option<&'a str>,
}

/// Token refresh request
#[derive(Debug, Serialize)]
pub struct RefreshTokenRequest<'a> {
    pub refresh_token: &'a str,
}

/// Two-factor setup response
#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorSetup {
    pub secret: String,
    pub qr_code: String,
    pub recovery_codes: Vec<String>,
}

/// Two-factor verification request
#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorVerify {
    pub code: String,
}

/// CAPTCHA response
#[derive(Debug, Deserialize)]
pub struct CaptchaResponse {
    pub image: String,
    pub ticket: String,
}

/// Login preparation data
#[derive(Debug, Deserialize)]
pub struct LoginPreparation {
    pub webauthn_enabled: bool,
    pub sso_enabled: bool,
    pub password_enabled: bool,
    pub qq_enabled: bool,
}

/// OpenID preparation request
#[derive(Debug, Serialize)]
pub struct OpenIdPrepareRequest<'a> {
    pub hint: Option<&'a str>,
    pub linking: Option<bool>,
    pub provider: i32,
}

/// OpenID finish request
#[derive(Debug, Serialize)]
pub struct OpenIdFinishRequest<'a> {
    pub code: &'a str,
    pub session_id: &'a str,
    pub provider_id: i32,
}

/// Passkey sign-in preparation
#[derive(Debug, Deserialize)]
pub struct PasskeySignInPreparation {
    pub session_id: String,
    pub options: Value,
}

/// Passkey sign-in request
#[derive(Debug, Serialize)]
pub struct PasskeySignInRequest<'a> {
    pub response: &'a str,
    pub session_id: &'a str,
}

/// Complete login response
#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub user: NewUser,
    pub token: Token,
}

/// Register request
#[derive(Debug, Serialize)]
pub struct RegisterRequest<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub email: Option<&'a str>,
}
