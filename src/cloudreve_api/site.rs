//! Site configuration for CloudreveAPI

use crate::Error;
use crate::api::v4::models::SiteConfigSection;
use crate::client::UnifiedClient;
use log::debug;

/// Unified site config value (handles V3 and V4 differences)
#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum SiteConfigValue {
    V3(crate::api::v3::models::SiteConfig),
    V4(Box<crate::api::v4::models::SiteConfig>),
}

/// Site configuration methods for CloudreveAPI
impl super::CloudreveAPI {
    /// Get site configuration (unified API for V3 and V4)
    ///
    /// V4: supports section parameter (basic, login, explorer, etc.)
    /// V3: ignores section, returns all config
    pub async fn get_site_config(&self, section: Option<&str>) -> Result<SiteConfigValue, Error> {
        debug!("Getting site config: section={:?}", section);

        match &self.inner {
            UnifiedClient::V4(client) => {
                let section_enum = section
                    .and_then(|s| match s {
                        "basic" => Some(SiteConfigSection::Basic),
                        "login" => Some(SiteConfigSection::Login),
                        "explorer" => Some(SiteConfigSection::Explorer),
                        "emojis" => Some(SiteConfigSection::Emojis),
                        "vas" => Some(SiteConfigSection::Vas),
                        "app" => Some(SiteConfigSection::App),
                        "thumb" => Some(SiteConfigSection::Thumb),
                        _ => None,
                    })
                    .unwrap_or(SiteConfigSection::Basic);

                let config = client.get_site_config(section_enum).await?;
                Ok(SiteConfigValue::V4(Box::new(config)))
            }
            UnifiedClient::V3(client) => {
                let config = client.get_site_config().await?;
                Ok(SiteConfigValue::V3(config))
            }
        }
    }
}
