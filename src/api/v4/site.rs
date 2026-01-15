//! Site-related API endpoints for Cloudreve API v4

use crate::api::v4::ApiV4Client;
use crate::api::v4::models::*;
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

    pub async fn ping(&self) -> Result<String, Error> {
        let response: crate::ApiResponse<String> = self.get("/site/ping").await?;
        Ok(response.msg)
    }

    pub async fn get_site_config(&self, section: &str) -> Result<SiteConfig, Error> {
        let endpoint = format!("/site/config/{}", section);
        let response: crate::ApiResponse<SiteConfig> = self.get(&endpoint).await?;
        match response.data {
            Some(config) => Ok(config),
            None => Err(crate::Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }

    pub async fn report_site_abuse(&self, request: &AbuseReportRequest<'_>) -> Result<(), Error> {
        let response: crate::ApiResponse<()> = self.post("/site/abuse", request).await?;
        if response.code == 0 {
            Ok(())
        } else {
            Err(crate::Error::Api {
                code: response.code,
                message: response.msg,
            })
        }
    }

    pub async fn get_captcha(&self) -> Result<CaptchaResponse, Error> {
        self.get("/site/captcha").await
    }
}
