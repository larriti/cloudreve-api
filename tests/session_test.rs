use cloudreve_api::api::v4::models::*;
use cloudreve_api::{CloudreveClient, Result};

#[cfg(test)]
mod session_tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() -> Result<()> {
        let client = CloudreveClient::new("https://example.com");
        assert_eq!(client.base_url, "https://example.com");
        Ok(())
    }

    #[tokio::test]
    async fn test_login_request_struct() -> Result<()> {
        let _login_request = LoginRequest {
            email: "test@example.com",
            password: "password",
        };
        Ok(())
    }

    #[tokio::test]
    async fn test_2fa_request_struct() -> Result<()> {
        let _2fa_request = TwoFactorLoginRequest {
            email: "test@example.com",
            password: "password",
            code: "123456",
            ticket: None,
        };
        Ok(())
    }

    #[tokio::test]
    async fn test_refresh_token_request_struct() -> Result<()> {
        let _refresh_request = RefreshTokenRequest {
            refresh_token: "refresh_token",
        };
        Ok(())
    }

    #[test]
    fn test_struct_definitions() {
        let _user = User {
            id: "1".to_string(),
            email: "test@example.com".to_string(),
            nickname: "Test User".to_string(),
            status: Some("active".to_string()),
            avatar: Some("avatar.jpg".to_string()),
            created_at: "2023-01-01T00:00:00Z".to_string(),
            group: Some(UserGroup {
                id: "1".to_string(),
                name: "Default".to_string(),
                permission: Some("read".to_string()),
                direct_link_batch_size: Some(10),
                trash_retention: Some(30),
            }),
        };

        let _token = Token {
            access_token: "access_token".to_string(),
            refresh_token: "refresh_token".to_string(),
            access_expires: "2023-01-02T00:00:00Z".to_string(),
            refresh_expires: "2023-02-01T00:00:00Z".to_string(),
        };

        let _login_data = LoginData {
            user: _user.clone(),
            token: _token.clone(),
        };

        let _site_config = SiteConfig {
            instance_id: Some("test-instance".to_string()),
            title: Some("Cloudreve".to_string()),
            login_captcha: Some(false),
            reg_captcha: Some(false),
            forget_captcha: Some(false),
            abuse_report_captcha: Some(false),
            themes: Some("default".to_string()),
            default_theme: Some("default".to_string()),
            authn: Some(false),
            user: None,
            captcha_re_captcha_key: None,
            captcha_cap_instance_url: "".to_string(),
            captcha_cap_site_key: "".to_string(),
            site_notice: None,
            captcha_type: None,
            turnstile_site_id: None,
            register_enabled: Some(true),
            qq_enabled: Some(false),
            sso_enabled: Some(false),
            sso_display_name: None,
            sso_icon: None,
            oidc_enabled: Some(false),
            oidc_display_name: None,
            oidc_icon: None,
            logo: None,
            logo_light: None,
            tos_url: None,
            privacy_policy_url: None,
            icons: None,
            emoji_preset: None,
            point_enabled: Some(false),
            share_point_gain_rate: None,
            map_provider: None,
            google_map_tile_type: None,
            file_viewers: None,
            max_batch_size: None,
            app_promotion: None,
            app_feedback: None,
            app_forum: None,
            payment: None,
            anonymous_purchase: None,
            point_price: None,
            shop_nav_enabled: None,
            storage_products: None,
            group_skus: None,
            thumbnail_width: None,
            thumbnail_height: None,
            custom_props: None,
            custom_nav_items: None,
            custom_html: None,
            mapbox_ak: None,
            thumb_exts: None,
        };

        let _new_user = NewUser {
            id: "1".to_string(),
            email: Some("test@example.com".to_string()),
            nickname: Some("Test User".to_string()),
            created_at: "2023-01-01T00:00:00Z".to_string(),
            anonymous: Some(false),
            group: Some(NewGroup {
                id: "1".to_string(),
                name: "Default".to_string(),
                permission: "read".to_string(),
                direct_link_batch_size: 10,
                trash_retention: 30,
            }),
            status: Some(UserStatus::Active),
            avatar: Some(AvatarType::Gravatar),
            preferred_theme: Some("default".to_string()),
            credit: Some(100),
            language: Some("en".to_string()),
            disable_view_sync: Some("false".to_string()),
            share_links_in_profile: Some(ShareLinkVisibility::AllShare),
        };
    }
}
