use cloudreve_api::{CloudreveClient, Result};
use tokio;

#[cfg(test)]
mod client_tests {
    use super::*;

    #[tokio::test]
    async fn test_client_initialization() -> Result<()> {
        let client = CloudreveClient::new("https://example.com");

        assert_eq!(client.base_url, "https://example.com");
        assert!(client.token.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_token_management() -> Result<()> {
        let mut client = CloudreveClient::new("https://example.com");

        // Test token setting
        client.set_token("test_token".to_string());
        assert_eq!(client.token, Some("test_token".to_string()));

        // Test token updating
        client.set_token("new_token".to_string());
        assert_eq!(client.token, Some("new_token".to_string()));

        Ok(())
    }

    #[tokio::test]
    async fn test_client_clone() -> Result<()> {
        let client = CloudreveClient::new("https://example.com");
        let cloned_client = client.clone();

        assert_eq!(client.base_url, cloned_client.base_url);

        Ok(())
    }

    #[test]
    fn test_debug_output() {
        let client = CloudreveClient::new("https://example.com");
        let debug_output = format!("{:?}", client);

        assert!(debug_output.contains("ApiV4Client"));
        assert!(debug_output.contains("https://example.com"));

        assert!(true);
    }

    #[tokio::test]
    async fn test_url_formatting() -> Result<()> {
        let client = CloudreveClient::new("https://example.com/");

        // We can't directly test the internal get_url method, but we can verify
        // that the client initializes correctly with trailing slash handling
        assert!(client.base_url.ends_with("/"));

        Ok(())
    }

    #[tokio::test]
    async fn test_client_with_different_base_urls() -> Result<()> {
        let client1 = CloudreveClient::new("https://example.com");
        let client2 = CloudreveClient::new("https://example.com/api");
        let client3 = CloudreveClient::new("https://subdomain.example.com/v1");

        assert_eq!(client1.base_url, "https://example.com");
        assert_eq!(client2.base_url, "https://example.com/api");
        assert_eq!(client3.base_url, "https://subdomain.example.com/v1");

        Ok(())
    }
}
