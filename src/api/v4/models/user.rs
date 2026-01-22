//! User management models for Cloudreve API v4

use serde::{Deserialize, Serialize};

/// Storage quota information
#[derive(Debug, Deserialize)]
pub struct Quota {
    pub used: u64,
    pub total: u64,
    #[serde(default)]
    pub storage_pack_total: Option<u64>,
}

/// User settings (preferences)
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UserSettings {
    /// Group expiration date
    #[serde(default)]
    pub group_expires: Option<String>,

    /// Linked OpenID providers
    #[serde(default)]
    pub open_id: Option<Vec<OpenIDInfo>>,

    /// Whether file version retention is enabled
    #[serde(default)]
    pub version_retention_enabled: bool,

    /// File extensions with version retention enabled
    #[serde(default)]
    pub version_retention_ext: Option<Vec<String>>,

    /// Max preserved versions (0 = all)
    #[serde(default)]
    pub version_retention_max: Option<i64>,

    /// Whether account is passwordless
    #[serde(default)]
    pub passwordless: bool,

    /// Whether 2FA is enabled
    #[serde(default)]
    pub two_fa_enabled: bool,

    /// Registered passkeys
    #[serde(default)]
    pub passkeys: Option<Vec<Passkey>>,

    /// Recent login activities
    #[serde(default)]
    pub login_activity: Option<Vec<LoginActivity>>,

    /// Storage packs
    #[serde(default)]
    pub storage_packs: Vec<StoragePack>,

    /// Available credit/points
    #[serde(default)]
    pub credit: i64,

    /// Whether view sync is disabled
    #[serde(default)]
    pub disable_view_sync: bool,

    /// Share link visibility in profile
    #[serde(default)]
    pub share_links_in_profile: Option<String>,
}

/// OpenID provider information
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct OpenIDInfo {
    #[serde(default)]
    pub provider: i32,
    #[serde(default)]
    pub linked_at: String,
}

/// Passkey information
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Passkey {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub used_at: Option<String>,
    #[serde(default)]
    pub created_at: String,
}

/// Login activity record
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct LoginActivity {
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub ip: String,
    #[serde(default)]
    pub browser: String,
    #[serde(default)]
    pub device: String,
    #[serde(default)]
    pub os: String,
    #[serde(default)]
    pub login_with: String,
    #[serde(default)]
    pub open_id_provider: i32,
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub webdav: bool,
}

/// Storage pack information
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct StoragePack {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub active_since: String,
    #[serde(default)]
    pub expire_at: String,
    #[serde(default)]
    pub size: i64,
}

/// Update profile request
#[derive(Debug, Serialize)]
pub struct UpdateProfileRequest<'a> {
    pub nickname: Option<&'a str>,
    pub email: Option<&'a str>,
    pub avatar: Option<&'a str>,
}

/// Change password request
#[derive(Debug, Serialize)]
pub struct ChangePasswordRequest<'a> {
    pub old_password: &'a str,
    pub new_password: &'a str,
}

/// Search user request
#[derive(Debug, Serialize)]
pub struct SearchUserRequest<'a> {
    pub query: &'a str,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

/// Update user setting request
#[derive(Debug, Serialize)]
pub struct UpdateUserSettingRequest<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

/// Credit change record
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditChangeRecord {
    pub id: String,
    pub amount: i64,
    pub reason: String,
    pub created_at: String,
}

/// Payment record
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentRecord {
    pub id: String,
    pub amount: f64,
    pub method: String,
    pub status: String,
    pub created_at: String,
    pub transaction_id: Option<String>,
}
