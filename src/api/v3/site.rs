//! Site-related API endpoints for Cloudreve API v3

use crate::api::v3::models::*;
use crate::api::v3::ApiV3Client;
use crate::api::VersionInfo;
use crate::Error;
use crate::VERSION;

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

    /// Ping the server and get server version
    pub async fn ping(&self) -> Result<String, Error> {
        let response: ApiResponse<String> = self.get("/site/ping").await?;
        match response.data {
            Some(version) => Ok(version),
            None => Err(Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }

    /// Get API version information
    pub async fn get_version(&self) -> Result<VersionInfo, Error> {
        let server_version = self.ping().await.unwrap_or_else(|_| "unknown".to_string());
        Ok(VersionInfo {
            api_version: "v3".to_string(),
            library_version: VERSION.to_string(),
            server_version,
        })
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
