//! Share operations for CloudreveAPI

use crate::Error;
use crate::api::v3::models as v3_models;
use crate::api::v4::models as v4_models;
use crate::client::UnifiedClient;
use log::debug;

/// Unified share item
#[derive(Debug, Clone)]
pub struct ShareItem {
    pub id: String,
    pub name: String,
    pub url: String,
    pub created_at: String,
    pub expired: bool,
}

/// Share methods for CloudreveAPI
impl super::CloudreveAPI {
    /// Create a share link for a file or directory
    ///
    /// Creates a share link with optional expiration and password.
    pub async fn create_share(
        &self,
        path: &str,
        _name: Option<&str>,
        expires_in: Option<u32>,
        password: Option<&str>,
    ) -> Result<String, Error> {
        debug!("Creating share link for: {}", path);

        match &self.inner {
            UnifiedClient::V3(client) => {
                let request = v3_models::ShareRequest {
                    id: path.to_string(),
                    is_dir: path.ends_with('/'),
                    password: password.unwrap_or("").to_string(),
                    downloads: 0,
                    expire: expires_in.unwrap_or(0) as i32,
                    preview: true,
                };
                let share = client.create_share(&request).await?;
                Ok(share.key)
            }
            UnifiedClient::V4(client) => {
                let permissions = v4_models::PermissionSetting {
                    user_explicit: serde_json::json!({}),
                    group_explicit: serde_json::json!({}),
                    same_group: "read".to_string(),
                    other: "read".to_string(),
                    anonymous: "read".to_string(),
                    everyone: "read".to_string(),
                };
                let request = v4_models::CreateShareLinkRequest {
                    permissions,
                    uri: path.to_string(),
                    is_private: Some(password.is_some()),
                    share_view: None,
                    expire: expires_in,
                    price: None,
                    password: password.map(|p| p.to_string()),
                    show_readme: None,
                };
                let share = client.create_share_link(&request).await?;
                Ok(share)
            }
        }
    }

    /// List all shares
    ///
    /// Returns a list of all share links for the current user.
    pub async fn list_shares(&self) -> Result<Vec<ShareItem>, Error> {
        debug!("Listing shares");

        match &self.inner {
            UnifiedClient::V3(_client) => {
                // V3 doesn't have a dedicated list shares endpoint
                // Return empty for now or implement via workarounds
                Ok(Vec::new())
            }
            UnifiedClient::V4(client) => {
                let shares = client.list_my_share_links().await?;
                Ok(shares
                    .into_iter()
                    .map(|s| ShareItem {
                        id: s.id,
                        name: s.name,
                        url: s.url,
                        created_at: s.created_at,
                        expired: s.expired,
                    })
                    .collect())
            }
        }
    }

    /// Update a share link
    ///
    /// Updates an existing share link with new settings.
    pub async fn update_share(&self, id: &str, props: &ShareUpdateProps) -> Result<(), Error> {
        debug!("Updating share: {}", id);

        match &self.inner {
            UnifiedClient::V3(_client) => Err(Error::UnsupportedFeature(
                "share update".to_string(),
                "v3".to_string(),
            )),
            UnifiedClient::V4(client) => {
                let permissions = v4_models::PermissionSetting {
                    user_explicit: serde_json::json!({}),
                    group_explicit: serde_json::json!({}),
                    same_group: "read".to_string(),
                    other: "read".to_string(),
                    anonymous: "read".to_string(),
                    everyone: "read".to_string(),
                };
                let request = v4_models::EditShareLinkRequest {
                    permissions,
                    uri: String::new(), // Will be filled by the API
                    share_view: None,
                    expire: props.expires,
                    price: None,
                    show_readme: None,
                };
                client.edit_share_link(id, &request).await?;
                Ok(())
            }
        }
    }

    /// Delete a share link
    ///
    /// Deletes an existing share link.
    pub async fn delete_share(&self, id: &str) -> Result<(), Error> {
        debug!("Deleting share: {}", id);

        match &self.inner {
            UnifiedClient::V3(_client) => Err(Error::UnsupportedFeature(
                "share deletion".to_string(),
                "v3".to_string(),
            )),
            UnifiedClient::V4(client) => {
                client.delete_share_link(id).await?;
                Ok(())
            }
        }
    }
}

/// Properties for updating a share
#[derive(Debug, Clone, Default)]
pub struct ShareUpdateProps {
    pub password: Option<String>,
    pub expires: Option<u32>,
}
