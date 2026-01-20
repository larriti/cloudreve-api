//! Share-related API endpoints for Cloudreve API v3

use crate::Error;
use crate::api::v3::ApiV3Client;
use crate::api::v3::models::*;

impl ApiV3Client {
    /// Create a share link
    /// Note: The V3 API may return either an ApiResponse<Share> object or a plain string URL
    pub async fn create_share(&self, request: &ShareRequest) -> Result<Share, Error> {
        // Get raw response to handle both formats
        let raw_text = self.post_raw("/share", request).await?;

        // Try to parse as ApiResponse<Share> first
        if let Ok(api_response) = serde_json::from_str::<ApiResponse<Share>>(&raw_text) {
            if let Some(share) = api_response.data {
                return Ok(share);
            }
            return Err(Error::Api {
                code: api_response.code,
                message: api_response.msg,
            });
        }

        // If that fails, try to parse as plain string URL
        let url = raw_text.trim();
        // Extract key from URL (e.g., "https://example.com/s/abc123" -> "abc123")
        let key = url.split('/').next_back().unwrap_or("").to_string();
        Ok(Share {
            key,
            ..Default::default()
        })
    }
}
