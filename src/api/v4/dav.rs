//! WebDAV account management API endpoints for Cloudreve v4

use crate::api::v4::models::*;
use crate::api::v4::uri::path_to_uri;
use crate::api::v4::ApiV4Client;
use crate::Error;

/// WebDAV account management methods
impl ApiV4Client {
    /// List WebDAV accounts
    pub async fn list_dav_accounts(
        &self,
        page_size: u32,
        next_page_token: Option<&str>,
    ) -> Result<DavAccountsResponse, Error> {
        let mut url = format!("/devices/dav?page_size={}", page_size);
        if let Some(token) = next_page_token {
            url.push_str(&format!("&next_page_token={}", token));
        }

        let response: ApiResponse<DavAccountsResponse> = self.get(&url).await?;
        match response.data {
            Some(data) => Ok(data),
            None => Err(Error::InvalidResponse(format!(
                "API returned no data for list_dav_accounts request: {:?}",
                response
            ))),
        }
    }

    /// Create a new WebDAV account
    pub async fn create_dav_account(
        &self,
        request: &CreateDavAccountRequest,
    ) -> Result<DavAccount, Error> {
        // Convert URI format internally
        let uri = path_to_uri(&request.uri);
        let converted_request = CreateDavAccountRequest {
            uri,
            name: request.name.clone(),
            readonly: request.readonly,
            proxy: request.proxy,
            disable_sys_files: request.disable_sys_files,
        };

        let response: ApiResponse<DavAccount> =
            self.put("/devices/dav", &converted_request).await?;
        match response.data {
            Some(data) => Ok(data),
            None => Err(Error::InvalidResponse(format!(
                "API returned no data for create_dav_account request: {:?}",
                response
            ))),
        }
    }

    /// Update a WebDAV account
    pub async fn update_dav_account(
        &self,
        id: &str,
        request: &CreateDavAccountRequest,
    ) -> Result<DavAccount, Error> {
        // Convert URI format internally
        let uri = path_to_uri(&request.uri);
        let converted_request = CreateDavAccountRequest {
            uri,
            name: request.name.clone(),
            readonly: request.readonly,
            proxy: request.proxy,
            disable_sys_files: request.disable_sys_files,
        };

        let response: ApiResponse<DavAccount> =
            self.patch(&format!("/devices/dav/{}", id), &converted_request).await?;
        match response.data {
            Some(data) => Ok(data),
            None => Err(Error::InvalidResponse(format!(
                "API returned no data for update_dav_account request: {:?}",
                response
            ))),
        }
    }

    /// Delete a WebDAV account
    pub async fn delete_dav_account(&self, id: &str) -> Result<(), Error> {
        let _: ApiResponse<()> = self.delete(&format!("/devices/dav/{}", id)).await?;
        Ok(())
    }
}
