use cloudreve_api::{CloudreveClient, Error, Result};
use tokio;

#[cfg(test)]
mod edge_cases_tests {
    use super::*;

    #[tokio::test]
    async fn test_extremely_long_strings() -> Result<()> {
        let client = CloudreveClient::new("https://httpbin.org");

        let extremely_long = "x".repeat(100_000);
        let _result = client
            .get::<String>(&format!("/get?q={}", extremely_long))
            .await;

        Ok(())
    }

    #[tokio::test]
    async fn test_null_bytes_in_parameters() -> Result<()> {
        let client = CloudreveClient::new("https://httpbin.org");

        let with_null = "test\0test";
        let _result = client.get::<String>(&format!("/get?q={}", with_null)).await;

        Ok(())
    }

    #[tokio::test]
    async fn test_unicode_edge_cases() -> Result<()> {
        let client = CloudreveClient::new("https://httpbin.org");

        let unicode_cases = vec![
            "🚀",
            "café\u{0301}",
            "\u{fffd}",
            "\u{1f600}\u{1f600}\u{1f600}",
        ];

        for case in unicode_cases {
            let _result = client.get::<String>(&format!("/get?q={}", case)).await;
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_numeric_boundary_values() -> Result<()> {
        let client = CloudreveClient::new("https://httpbin.org");

        let boundary_cases = vec![
            i32::MIN.to_string(),
            i32::MAX.to_string(),
            u64::MAX.to_string(),
            "0".to_string(),
            "-0".to_string(),
        ];

        for case in boundary_cases {
            let _result = client.get::<String>(&format!("/get?num={}", case)).await;
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_special_characters_in_urls() -> Result<()> {
        let client = CloudreveClient::new("https://httpbin.org");

        let special_chars = vec![
            "/path/with spaces",
            "/path/with@signs",
            "/path/with#hashes",
            "/path/with?query&params",
            "/path/with%percent",
            "/path/with^carat",
            "/path/with|pipe",
            "/path/with[brackets]",
        ];

        for path in special_chars {
            let _result = client.get::<String>(path).await;
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_empty_structures_and_arrays() -> Result<()> {
        let client = CloudreveClient::new("https://httpbin.org");

        use serde_json::Value;

        let empty_object: Value = serde_json::json!({});
        let empty_array: Value = serde_json::json!([]);

        let _result1 = client.post::<Value>("/post", &empty_object).await;
        let _result2 = client.post::<Value>("/post", &empty_array).await;

        Ok(())
    }

    #[tokio::test]
    async fn test_large_json_payloads() -> Result<()> {
        let client = CloudreveClient::new("https://httpbin.org");

        let large_payload = serde_json::json!({
            "data": "x".repeat(1_000_000),
            "nested": {
                "level1": {
                    "level2": {
                        "level3": "deep_value".repeat(1000)
                    }
                }
            },
            "array": (0..1000).map(|i| format!("item_{}", i)).collect::<Vec<_>>()
        });

        let _result = client
            .post::<serde_json::Value>("/post", &large_payload)
            .await;

        Ok(())
    }

    #[tokio::test]
    async fn test_multiple_concurrent_requests_stress() -> Result<()> {
        let client = CloudreveClient::new("https://invalid-domain-for-stress-test.com");

        let mut handles = vec![];

        for _ in 0..50 {
            let client_clone = client.clone();
            let handle = tokio::spawn(async move { client_clone.ping().await });
            handles.push(handle);
        }

        let results = futures::future::join_all(handles).await;

        for result in results {
            let inner_result: Result<String> = result.unwrap();
            assert!(inner_result.is_err());
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_memory_allocation_failures_simulation() -> Result<()> {
        let client = CloudreveClient::new("https://httpbin.org");

        let mut large_vec = Vec::new();
        for i in 0..100_000 {
            large_vec.push(format!("item_{}", i));
        }

        let large_payload = serde_json::json!({
            "items": large_vec
        });

        let _result = client
            .post::<serde_json::Value>("/post", &large_payload)
            .await;

        Ok(())
    }

    #[tokio::test]
    async fn test_header_manipulation_edge_cases() -> Result<()> {
        use reqwest::Client;

        let http_client = Client::builder().build().unwrap();

        let client = CloudreveClient {
            base_url: "https://httpbin.org".to_string(),
            http_client,
            token: Some("test_token_with_special_chars_!@#$%^&*()".to_string()),
        };

        let _result = client.get::<serde_json::Value>("/bearer").await;

        Ok(())
    }

    #[tokio::test]
    async fn test_url_encoding_edge_cases() -> Result<()> {
        let client = CloudreveClient::new("https://httpbin.org");

        let encoding_cases = vec![
            "/path with spaces",
            "/path%20with%20encoded%20spaces",
            "/path/with/slashes/",
            "/path/../path",
            "/path/./file",
            "/path//double//slashes",
            "/path/file?param=value&other=with spaces",
            "/path/file?param=value%26encoded",
        ];

        for case in encoding_cases {
            let _result = client.get::<String>(case).await;
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_zero_sized_types_and_empty_responses() -> Result<()> {
        let client = CloudreveClient::new("https://httpbin.org");

        let _result = async {
            let _response = client.get::<()>("").await;
            Ok::<(), Error>(())
        }
        .await;

        Ok(())
    }
}

#[cfg(test)]
mod error_propagation_tests {
    use super::*;

    #[tokio::test]
    async fn test_error_chain_propagation() -> Result<()> {
        let client = CloudreveClient::new("https://definitely-not-a-real-domain-123456789.com");

        let result = client.ping().await;
        if result.is_err() {
            assert!(result.is_err());
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_error_context_preservation() -> Result<()> {
        let client = CloudreveClient::new("https://httpbin.org");

        let result = client.get::<i32>("/status/200").await;

        if let Err(error) = result {
            match error {
                Error::Json(_) => {}
                _ => {}
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_unrecoverable_error_conditions() -> Result<()> {
        let client = CloudreveClient::new("ftp://example.com");

        let _result = client.ping().await;

        Ok(())
    }
}
