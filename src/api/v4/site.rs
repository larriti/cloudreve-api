//! Site-related API endpoints for Cloudreve API v4

use crate::api::v4::ApiV4Client;
use crate::api::VersionInfo;
use crate::Error;
use crate::VERSION;

impl ApiV4Client {
    /// Get API version information
    pub async fn get_version(&self) -> Result<VersionInfo, Error> {
        Ok(VersionInfo {
            api_version: "v4".to_string(),
            library_version: VERSION.to_string(),
            server_version: "unknown".to_string(),
        })
    }
}
