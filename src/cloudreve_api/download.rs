//! Download operations for CloudreveAPI

use crate::client::UnifiedClient;
use crate::api::v4::models as v4_models;
use crate::Error;
use log::debug;

/// Download methods for CloudreveAPI
impl super::CloudreveAPI {
    /// Create a download URL for a file
    ///
    /// Returns a download URL that can be used to download the file.
    pub async fn create_download_url(&self, path: &str) -> Result<String, Error> {
        debug!("Creating download URL for: {}", path);

        match &self.inner {
            UnifiedClient::V3(client) => {
                // V3: Need to get file ID first, then get download URL
                let url = client.download_file(path).await?;
                Ok(url.url)
            }
            UnifiedClient::V4(client) => {
                let request = v4_models::CreateDownloadUrlRequest {
                    uris: vec![path],
                    download: Some(true),
                    redirect: Some(true),
                    entity: None,
                    use_primary_site_url: None,
                    skip_error: None,
                    archive: None,
                    no_cache: None,
                };
                let response = client.create_download_url(&request).await?;
                // Return the first URL
                if let Some(first_url) = response.urls.first() {
                    Ok(first_url.url.clone())
                } else {
                    Err(Error::InvalidResponse("No download URL returned".to_string()))
                }
            }
        }
    }
}
