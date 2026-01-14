//! Site-related API endpoints for Cloudreve API v3

use crate::api::v3::models::*;
use crate::api::v3::ApiV3Client;
use crate::Error;

impl ApiV3Client {
    /// Get site configuration
    pub async fn get_site_config(&self) -> Result<SiteConfig, Error> {
        let response: ApiResponse<SiteConfig> = self.get("/site/config").await?;
        match response.data {
            Some(config) => Ok(config),
            None => Err(Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }

    /// Get user storage information
    pub async fn get_user_storage(&self) -> Result<StorageInfo, Error> {
        let response: ApiResponse<StorageInfo> = self.get("/user/storage").await?;
        match response.data {
            Some(storage) => Ok(storage),
            None => Err(Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }

    /// Ping the server
    pub async fn ping(&self) -> Result<String, Error> {
        let response: ApiResponse<String> = self.get("/site/ping").await?;
        Ok(response.msg)
    }

    /// Get user settings
    pub async fn get_user_settings(&self) -> Result<StorageInfo, Error> {
        let response: ApiResponse<StorageInfo> = self.get("/user/setting").await?;
        match response.data {
            Some(settings) => Ok(settings),
            None => Err(Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }

    /// Get task queue
    pub async fn get_task_queue(&self) -> Result<Vec<Aria2Task>, Error> {
        let response: ApiResponse<Vec<Aria2Task>> = self.get("/user/setting/tasks").await?;
        match response.data {
            Some(tasks) => Ok(tasks),
            None => Err(Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }
}
