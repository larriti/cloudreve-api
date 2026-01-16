//! Site configuration models for Cloudreve API v4

use serde::{Deserialize, Serialize};

/// Site configuration
#[derive(Debug, Deserialize)]
pub struct SiteConfig {
    pub instance_id: Option<String>,
    pub title: Option<String>,
    pub login_captcha: Option<bool>,
    pub reg_captcha: Option<bool>,
    pub forget_captcha: Option<bool>,
    pub abuse_report_captcha: Option<bool>,
    pub themes: Option<String>,
    pub default_theme: Option<String>,
    pub authn: Option<bool>,
    pub user: Option<super::auth::NewUser>,
    pub captcha_re_captcha_key: Option<String>,
    pub captcha_cap_instance_url: String,
    pub captcha_cap_site_key: String,
    pub site_notice: Option<String>,
    pub captcha_type: Option<String>,
    pub turnstile_site_id: Option<String>,
    pub register_enabled: Option<bool>,
    pub qq_enabled: Option<bool>,
    pub sso_enabled: Option<bool>,
    pub sso_display_name: Option<String>,
    pub sso_icon: Option<String>,
    pub oidc_enabled: Option<bool>,
    pub oidc_display_name: Option<String>,
    pub oidc_icon: Option<String>,
    pub logo: Option<String>,
    pub logo_light: Option<String>,
    pub tos_url: Option<String>,
    pub privacy_policy_url: Option<String>,
    pub icons: Option<String>,
    pub emoji_preset: Option<String>,
    pub point_enabled: Option<bool>,
    pub share_point_gain_rate: Option<f64>,
    pub map_provider: Option<String>,
    pub google_map_tile_type: Option<String>,
    pub file_viewers: Option<Vec<FileViewer>>,
    pub max_batch_size: Option<f64>,
    pub app_promotion: Option<bool>,
    pub app_feedback: Option<String>,
    pub app_forum: Option<String>,
    pub payment: Option<PaymentSetting>,
    pub anonymous_purchase: Option<bool>,
    pub point_price: Option<f64>,
    pub shop_nav_enabled: Option<bool>,
    pub storage_products: Option<Vec<StorageProduct>>,
    pub group_skus: Option<Vec<GroupSKU>>,
    pub thumbnail_width: Option<f64>,
    pub thumbnail_height: Option<f64>,
    pub custom_props: Option<Vec<CustomProps>>,
    pub custom_nav_items: Option<Vec<CustomNavItem>>,
    pub custom_html: Option<CustomHTML>,
    pub mapbox_ak: Option<String>,
    pub thumb_exts: Option<Vec<String>>,
}

/// File viewer configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct FileViewer {
    pub extensions: Vec<String>,
    pub handler: String,
    pub name: String,
    pub priority: i32,
}

/// Payment setting
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentSetting {
    pub providers: Vec<PaymentProvider>,
}

/// Payment provider
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentProvider {
    pub id: String,
    pub name: String,
    pub enabled: bool,
}

/// Storage product
#[derive(Debug, Serialize, Deserialize)]
pub struct StorageProduct {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub storage: i64,
}

/// Group SKU
#[derive(Debug, Serialize, Deserialize)]
pub struct GroupSKU {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub group_id: String,
}

/// Custom property
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomProps {
    pub key: String,
    pub name: String,
    pub r#type: String,
    pub options: Option<Vec<String>>,
}

/// Custom navigation item
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomNavItem {
    pub icon: String,
    pub name: String,
    pub url: String,
}

/// Custom HTML
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomHTML {
    pub head: Option<String>,
    pub body: Option<String>,
}
