//! User-related API endpoints for Cloudreve API v3

use crate::Error;
use crate::api::v3::ApiV3Client;
use crate::api::v3::models::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct WebdavListWrapper {
    accounts: Vec<WebdavAccount>,
}

impl ApiV3Client {
    /// Get WebDAV accounts
    pub async fn get_webdav_accounts(&self) -> Result<Vec<WebdavAccount>, Error> {
        let response: ApiResponse<WebdavListWrapper> = self.get("/webdav/accounts").await?;
        match response.data {
            Some(wrapper) => Ok(wrapper.accounts),
            None => Err(Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }
}
