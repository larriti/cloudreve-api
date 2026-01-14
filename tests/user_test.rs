use cloudreve_api::api::v4::models::*;
use cloudreve_api::{CloudreveClient, Result};
use tokio;

#[cfg(test)]
mod user_tests {
    use super::*;

    #[tokio::test]
    async fn test_register_request_struct() -> Result<()> {
        let _register_request = RegisterRequest {
            username: "testuser",
            password: "password123",
            email: Some("test@example.com"),
        };
        Ok(())
    }

    #[tokio::test]
    async fn test_update_profile_request_struct() -> Result<()> {
        let _profile_request = UpdateProfileRequest {
            nickname: Some("New Name"),
            email: Some("newemail@example.com"),
            avatar: Some("avatar_url"),
        };
        Ok(())
    }

    #[tokio::test]
    async fn test_change_password_request_struct() -> Result<()> {
        let _change_password_request = ChangePasswordRequest {
            old_password: "old_password",
            new_password: "new_password",
        };
        Ok(())
    }

    #[test]
    fn test_user_structs() {
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

        let _quota = Quota {
            used: 1024,
            total: 1024000,
            storage_pack_total: None,
        };

        let _settings = UserSettings {
            theme: Some("default".to_string()),
            language: Some("en".to_string()),
            timezone: Some("UTC".to_string()),
        };

        let _credit_record = CreditChangeRecord {
            id: "1".to_string(),
            amount: 100,
            reason: "Test credit".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
        };

        let _payment_record = PaymentRecord {
            id: "1".to_string(),
            amount: 10.0,
            method: "credit".to_string(),
            status: "completed".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
            transaction_id: Some("tx123".to_string()),
        };

        let _two_factor_setup = TwoFactorSetup {
            secret: "secret123".to_string(),
            qr_code: "qr_data".to_string(),
            recovery_codes: vec!["code1".to_string(), "code2".to_string()],
        };

        let _two_factor_verify = TwoFactorVerify {
            code: "123456".to_string(),
        };

        assert!(true);
    }

    #[test]
    fn test_storage_policy_parsing() {
        // Test JSON from API documentation
        let json = r#"{
            "code": 0,
            "data": [
                {
                    "id": "B1Fy",
                    "name": "Minio",
                    "type": "s3",
                    "max_size": 0
                },
                {
                    "id": "mqHp",
                    "name": "Cloudflare R2",
                    "type": "s3",
                    "max_size": 0
                },
                {
                    "id": "0dsD",
                    "name": "Google Cloud Storage",
                    "type": "s3",
                    "max_size": 0
                },
                {
                    "id": "NrTZ",
                    "name": "Qiniu",
                    "type": "qiniu",
                    "max_size": 0,
                    "relay": true
                }
            ],
            "msg": ""
        }"#;

        let response: ApiResponse<Vec<StoragePolicy>> = serde_json::from_str(json).unwrap();

        assert_eq!(response.code, 0);
        assert!(response.data.is_some());
        let policies = response.data.unwrap();
        assert_eq!(policies.len(), 4);

        assert_eq!(policies[0].id, "B1Fy");
        assert_eq!(policies[0].name, "Minio");
        assert_eq!(policies[0].type_, "s3");
        assert_eq!(policies[0].max_size, 0);

        assert_eq!(policies[3].id, "NrTZ");
        assert_eq!(policies[3].name, "Qiniu");
        assert_eq!(policies[3].type_, "qiniu");
        assert_eq!(policies[3].relay, Some(true));
    }
}
