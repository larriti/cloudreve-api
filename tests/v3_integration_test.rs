use cloudreve_api::api::v3::models::*;
use cloudreve_api::{ApiV3Client, Result};
use tokio;

#[cfg(test)]
mod v3_integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_client_lifecycle() -> Result<()> {
        let mut client = ApiV3Client::new("https://example.com");

        assert_eq!(client.base_url, "https://example.com");
        assert!(client.session_cookie.is_none());

        client.set_session_cookie("test-session-token".to_string());
        assert_eq!(
            client.session_cookie,
            Some("test-session-token".to_string())
        );

        Ok(())
    }

    #[test]
    fn test_all_request_structs() -> Result<()> {
        let _login = LoginRequest {
            user_name: "user@example.com",
            password: "password",
            captcha_code: "",
        };

        let _otp = OtpLoginRequest {
            code: "123456".to_string(),
        };

        let _upload = UploadFileRequest {
            path: "/test",
            size: 1024,
            name: "file.txt",
            policy_id: "policy",
            last_modified: 1234567890,
            mime_type: "text/plain",
        };

        let _share = ShareRequest {
            id: "file123".to_string(),
            is_dir: false,
            password: "".to_string(),
            downloads: -1,
            expire: 86400,
            preview: true,
        };

        let _aria2 = Aria2CreateRequest {
            dst: "/downloads",
            url: vec!["https://example.com/file.zip"],
        };

        Ok(())
    }

    #[test]
    fn test_serialization_compatibility() -> Result<()> {
        let user = User {
            id: "1".to_string(),
            user_name: "test@example.com".to_string(),
            nickname: "Test".to_string(),
            status: 0,
            avatar: "".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            preferred_theme: "default".to_string(),
            anonymous: false,
            group: UserGroup {
                id: 1,
                name: "Default".to_string(),
                allow_share: true,
                allow_remote_download: true,
                allow_archive_download: true,
                share_download: true,
                compress: true,
                webdav: true,
                source_batch: 1000,
                advance_delete: true,
                allow_web_dav_proxy: false,
            },
            tags: vec![],
        };

        let _json = serde_json::to_string(&user).unwrap();
        let _decoded: User = serde_json::from_str(&_json).unwrap();

        assert_eq!(user.id, "1");

        Ok(())
    }
}
