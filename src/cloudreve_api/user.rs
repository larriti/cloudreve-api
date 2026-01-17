//! User management for CloudreveAPI

use crate::client::UnifiedClient;
use crate::Error;
use log::debug;

/// Unified user information
#[derive(Debug, Clone)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub nickname: String,
    pub group: Option<String>,
    pub status: Option<String>,
}

/// Unified storage quota
#[derive(Debug, Clone)]
pub struct StorageQuota {
    pub used: u64,
    pub total: u64,
    pub free: u64,
}

/// User management methods for CloudreveAPI
impl super::CloudreveAPI {
    /// Get user information
    ///
    /// Returns unified user information regardless of API version.
    pub async fn get_user_info(&self) -> Result<UserInfo, Error> {
        debug!("Getting user info");

        match &self.inner {
            UnifiedClient::V3(client) => {
                // V3: Get user info from site config
                let config = client.get_site_config().await?;
                if let Some(user) = config.user {
                    Ok(UserInfo {
                        id: user.id,
                        email: user.user_name,
                        nickname: user.nickname,
                        group: Some(user.group.name),
                        status: Some(user.status.to_string()),
                    })
                } else {
                    Err(Error::InvalidResponse("No user info in site config".to_string()))
                }
            }
            UnifiedClient::V4(_client) => {
                // V4: Use a placeholder - actual implementation would call user endpoint
                Err(Error::InvalidResponse(
                    "User info endpoint not yet implemented for V4".to_string()
                ))
            }
        }
    }

    /// Get storage quota information
    ///
    /// Returns unified storage quota regardless of API version.
    pub async fn get_storage_quota(&self) -> Result<StorageQuota, Error> {
        debug!("Getting storage quota");

        match &self.inner {
            UnifiedClient::V3(client) => {
                let storage = client.get_user_storage().await?;
                let used = storage.used as u64;
                let total = storage.total as u64;
                Ok(StorageQuota {
                    used,
                    total,
                    free: total.saturating_sub(used),
                })
            }
            UnifiedClient::V4(client) => {
                let quota = client.get_user_capacity().await?;
                Ok(StorageQuota {
                    used: quota.used,
                    total: quota.total,
                    free: quota.total.saturating_sub(quota.used),
                })
            }
        }
    }
}
