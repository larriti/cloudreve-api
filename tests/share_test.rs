use cloudreve_api::Result;
use cloudreve_api::api::v4::models::*;

#[cfg(test)]
mod share_tests {
    use super::*;

    #[tokio::test]
    async fn test_create_share_request_struct() -> Result<()> {
        let _create_request = CreateShareLinkRequest {
            permissions: PermissionSetting {
                user_explicit: serde_json::json!({}),
                group_explicit: serde_json::json!({}),
                same_group: "read".to_string(),
                other: "none".to_string(),
                anonymous: "none".to_string(),
                everyone: "read".to_string(),
            },
            uri: "/path/file.txt".to_string(),
            is_private: Some(false),
            share_view: Some(true),
            expire: Some(3600),
            price: Some(0),
            password: Some("password".to_string()),
            show_readme: Some(true),
        };
        Ok(())
    }

    #[tokio::test]
    async fn test_edit_share_request_struct() -> Result<()> {
        let _edit_request = EditShareLinkRequest {
            permissions: PermissionSetting {
                user_explicit: serde_json::json!({}),
                group_explicit: serde_json::json!({}),
                same_group: "read".to_string(),
                other: "none".to_string(),
                anonymous: "none".to_string(),
                everyone: "read".to_string(),
            },
            uri: "/path/file.txt".to_string(),
            share_view: Some(true),
            expire: Some(7200),
            price: Some(0),
            show_readme: Some(false),
        };
        Ok(())
    }

    #[tokio::test]
    async fn test_abuse_report_request_struct() -> Result<()> {
        let _abuse_request = AbuseReportRequest { reason: "spam" };
        Ok(())
    }

    #[test]
    fn test_share_structs() {
        let _permission_setting = PermissionSetting {
            user_explicit: serde_json::json!({}),
            group_explicit: serde_json::json!({}),
            same_group: "read".to_string(),
            other: "none".to_string(),
            anonymous: "none".to_string(),
            everyone: "read".to_string(),
        };

        let _share = ShareLink {
            id: "1".to_string(),
            name: "Test Share".to_string(),
            visited: 10,
            downloaded: 5,
            price: 0,
            unlocked: false,
            source_type: ShareSourceType::File,
            owner: NewUser {
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
            },
            created_at: "2023-01-01T00:00:00Z".to_string(),
            expired: false,
            url: "https://example.com/s/1".to_string(),
            permission_setting: Some(_permission_setting.clone()),
            is_private: Some(false),
            password: Some("password".to_string()),
            source_uri: Some("/path/file.txt".to_string()),
            share_view: Some(true),
            show_readme: Some(true),
            password_protected: Some(true),
            expires: Some("2024-01-01T00:00:00Z".to_string()),
            expired_at: Some("2024-01-01T00:00:00Z".to_string()),
            download_count: 5,
        };
    }
}
