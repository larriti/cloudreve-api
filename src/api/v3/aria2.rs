//! Aria2-related API endpoints for Cloudreve API v3

use crate::Error;
use crate::api::v3::ApiV3Client;
use crate::api::v3::models::*;

impl ApiV3Client {
    /// Create offline download
    pub async fn create_download(&self, request: &Aria2CreateRequest<'_>) -> Result<(), Error> {
        let response: ApiResponse<()> = self.post("/aria2/url", request).await?;
        if response.code == 0 {
            Ok(())
        } else {
            Err(Error::Api {
                code: response.code,
                message: response.msg,
            })
        }
    }

    /// List downloading tasks
    pub async fn list_downloading(&self) -> Result<Vec<Aria2Task>, Error> {
        let response: ApiResponse<Vec<Aria2Task>> = self.get("/aria2/downloading").await?;
        match response.data {
            Some(tasks) => Ok(tasks),
            None => Err(Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }

    /// List finished tasks
    pub async fn list_finished(&self) -> Result<Vec<Aria2Task>, Error> {
        let response: ApiResponse<Vec<Aria2Task>> = self.get("/aria2/finished").await?;
        match response.data {
            Some(tasks) => Ok(tasks),
            None => Err(Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }

    /// Delete task
    pub async fn delete_task(&self, gid: &str) -> Result<(), Error> {
        let response: ApiResponse<()> = self.delete(&format!("/aria2/task/{}", gid)).await?;
        if response.code == 0 {
            Ok(())
        } else {
            Err(Error::Api {
                code: response.code,
                message: response.msg,
            })
        }
    }
}
