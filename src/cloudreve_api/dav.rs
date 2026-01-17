//! WebDAV account operations for CloudreveAPI

use crate::client::UnifiedClient;
use crate::Error;
use log::debug;

/// Unified WebDAV account information
#[derive(Debug, Clone)]
pub struct DavAccount {
    pub id: String,
    pub name: String,
    pub uri: Option<String>,
    pub server: Option<String>,
    pub password: Option<String>,
    pub created_at: String,
}

/// Unified WebDAV list response
#[derive(Debug)]
pub struct DavListResponse {
    pub accounts: Vec<DavAccount>,
}

/// WebDAV operations methods for CloudreveAPI
impl super::CloudreveAPI {
    /// List WebDAV accounts
    ///
    /// Returns a unified list of WebDAV accounts regardless of API version.
    pub async fn list_dav_accounts(&self, page_size: u32) -> Result<DavListResponse, Error> {
        debug!("Listing WebDAV accounts with page_size: {}", page_size);

        match &self.inner {
            UnifiedClient::V3(client) => {
                let accounts = client.get_webdav_accounts().await?;
                let dav_accounts = accounts.into_iter().map(|acc| DavAccount {
                    id: acc.id.to_string(),
                    name: acc.name,
                    uri: None,
                    server: Some(acc.server),
                    password: None,
                    created_at: acc.created_at,
                }).collect();
                Ok(DavListResponse { accounts: dav_accounts })
            }
            UnifiedClient::V4(client) => {
                let response = client.list_dav_accounts(page_size, None).await?;
                let dav_accounts = response.accounts.into_iter().map(|acc| DavAccount {
                    id: acc.id.to_string(),
                    name: acc.name,
                    uri: Some(acc.uri),
                    server: None,
                    password: Some(acc.password),
                    created_at: acc.created_at,
                }).collect();
                Ok(DavListResponse { accounts: dav_accounts })
            }
        }
    }

    /// Create a WebDAV account
    ///
    /// Creates a new WebDAV account. Only available in V4.
    pub async fn create_dav_account(
        &self,
        uri: &str,
        name: &str,
        readonly: bool,
        proxy: bool,
    ) -> Result<(), Error> {
        debug!("Creating WebDAV account: {} at {}", name, uri);

        match &self.inner {
            UnifiedClient::V3(_) => {
                Err(Error::UnsupportedFeature(
                    "create WebDAV account".to_string(),
                    "v3".to_string(),
                ))
            }
            UnifiedClient::V4(client) => {
                let request = crate::api::v4::models::CreateDavAccountRequest {
                    uri: uri.to_string(),
                    name: name.to_string(),
                    readonly: Some(readonly),
                    proxy: Some(proxy),
                    disable_sys_files: None,
                };
                client.create_dav_account(&request).await?;
                Ok(())
            }
        }
    }

    /// Update a WebDAV account
    ///
    /// Updates an existing WebDAV account. Only available in V4.
    pub async fn update_dav_account(
        &self,
        id: &str,
        uri: Option<&str>,
        name: Option<&str>,
        readonly: Option<bool>,
        proxy: Option<bool>,
    ) -> Result<(), Error> {
        debug!("Updating WebDAV account: {}", id);

        match &self.inner {
            UnifiedClient::V3(_) => {
                Err(Error::UnsupportedFeature(
                    "update WebDAV account".to_string(),
                    "v3".to_string(),
                ))
            }
            UnifiedClient::V4(client) => {
                // For update, we need to get the current account first to fill in missing fields
                let current_list = client.list_dav_accounts(100, None).await?;
                let current = current_list.accounts.iter()
                    .find(|a| a.id == id)
                    .ok_or_else(|| Error::InvalidResponse(format!("WebDAV account '{}' not found", id)))?;

                let request = crate::api::v4::models::CreateDavAccountRequest {
                    uri: uri.unwrap_or(&current.uri).to_string(),
                    name: name.unwrap_or(&current.name).to_string(),
                    readonly,
                    proxy,
                    disable_sys_files: None,
                };
                client.update_dav_account(id, &request).await?;
                Ok(())
            }
        }
    }

    /// Delete a WebDAV account
    ///
    /// Deletes a WebDAV account. Only available in V4.
    pub async fn delete_dav_account(&self, id: &str) -> Result<(), Error> {
        debug!("Deleting WebDAV account: {}", id);

        match &self.inner {
            UnifiedClient::V3(_) => {
                Err(Error::UnsupportedFeature(
                    "delete WebDAV account".to_string(),
                    "v3".to_string(),
                ))
            }
            UnifiedClient::V4(client) => {
                client.delete_dav_account(id).await?;
                Ok(())
            }
        }
    }
}
