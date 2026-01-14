//! Directory-related API endpoints for Cloudreve API v3

use crate::api::v3::models::*;
use crate::api::v3::ApiV3Client;
use crate::Error;

impl ApiV3Client {
    /// List directory contents
    pub async fn list_directory(&self, path: &str) -> Result<DirectoryList, Error> {
        let encoded_path = urlencoding::encode(path);
        let response: ApiResponse<DirectoryList> =
            self.get(&format!("/directory{}", encoded_path)).await?;
        match response.data {
            Some(list) => Ok(list),
            None => Err(Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }

    /// Create a directory
    pub async fn create_directory(
        &self,
        request: &CreateDirectoryRequest<'_>,
    ) -> Result<(), Error> {
        let response: ApiResponse<()> = self.put("/directory", request).await?;
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
