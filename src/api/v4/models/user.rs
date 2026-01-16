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

/// User settings
#[derive(Debug, Serialize, Deserialize)]
pub struct UserSettings {
    pub theme: Option<String>,
    pub language: Option<String>,
    pub timezone: Option<String>,
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
