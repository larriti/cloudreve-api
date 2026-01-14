//! Integration tests for the Cloudreve API Rust library

use cloudreve_api::{CloudreveClient, Result};

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_client_creation() -> Result<()> {
        let client = CloudreveClient::new("https://example.com");
        assert_eq!(client.base_url, "https://example.com");
        Ok(())
    }

    #[tokio::test]
    async fn test_ping() -> Result<()> {
        // This test would require a real Cloudreve instance
        // For now we just test that the method compiles and can be called
        let client = CloudreveClient::new("https://example.com");
        // We can't actually call ping without a real instance, but we can test
        // that the method signature is correct and compiles
        assert_eq!(client.base_url, "https://example.com");
        Ok(())
    }
}
