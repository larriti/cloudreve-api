//! User-related API endpoints for Cloudreve API v3

use crate::api::v3::models::*;
use crate::api::v3::ApiV3Client;
use crate::Error;

impl ApiV3Client {
    /// Get WebDAV accounts
    pub async fn get_webdav_accounts(&self) -> Result<Vec<WebdavAccount>, Error> {
        let response: ApiResponse<Vec<WebdavAccount>> = self.get("/webdav/accounts").await?;
        match response.data {
            Some(accounts) => Ok(accounts),
            None => Err(Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }
}
