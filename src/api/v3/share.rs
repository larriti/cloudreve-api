//! Share-related API endpoints for Cloudreve API v3

use crate::api::v3::models::*;
use crate::api::v3::ApiV3Client;
use crate::Error;

impl ApiV3Client {
    /// Create a share link
    pub async fn create_share(&self, request: &ShareRequest) -> Result<Share, Error> {
        let response: ApiResponse<Share> = self.post("/share", request).await?;
        match response.data {
            Some(share) => Ok(share),
            None => Err(Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }
}
