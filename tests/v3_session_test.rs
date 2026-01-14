use cloudreve_api::api::v3::models::*;
use cloudreve_api::{ApiV3Client, Result};
use tokio;

#[cfg(test)]
mod v3_session_tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() -> Result<()> {
        let client = ApiV3Client::new("https://example.com");
        assert_eq!(client.base_url, "https://example.com");
        assert!(client.session_cookie.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_session_cookie() -> Result<()> {
        let mut client = ApiV3Client::new("https://example.com");
        client.set_session_cookie("test-cookie".to_string());
        assert_eq!(client.session_cookie, Some("test-cookie".to_string()));
        Ok(())
    }

    #[tokio::test]
    async fn test_url_construction() -> Result<()> {
        let client = ApiV3Client::new("https://example.com/");
        let url = client.get_url("/user/session");
        assert_eq!(url, "https://example.com/api/v3/user/session");
        Ok(())
    }

    #[test]
    fn test_login_request_struct() -> Result<()> {
        let _login_request = LoginRequest {
            user_name: "user@example.com",
            password: "password",
            captcha_code: "",
        };
        Ok(())
    }

    #[test]
    fn test_otp_login_request_struct() -> Result<()> {
        let _otp_request = OtpLoginRequest {
            code: "123456".to_string(),
        };
        Ok(())
    }
}
