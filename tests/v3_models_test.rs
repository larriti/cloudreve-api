use cloudreve_api::api::v3::models::*;

#[cfg(test)]
mod v3_models_tests {
    use super::*;

    #[test]
    fn test_user_struct() {
        let user = User {
            id: "l6hY".to_string(),
            user_name: "admin@cloudreve.org".to_string(),
            nickname: "admin".to_string(),
            status: 0,
            avatar: "".to_string(),
            created_at: "2024-05-01T11:04:25.490486+08:00".to_string(),
            preferred_theme: "".to_string(),
            anonymous: false,
            group: UserGroup {
                id: 1,
                name: "Admin".to_string(),
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

        assert_eq!(user.id, "l6hY");
        assert_eq!(user.user_name, "admin@cloudreve.org");
        assert_eq!(user.nickname, "admin");
        assert_eq!(user.status, 0);
    }

    #[test]
    fn test_object_struct() {
        let obj = Object {
            id: "bbTL".to_string(),
            name: "my_folder".to_string(),
            path: "/".to_string(),
            thumb: false,
            size: 0,
            object_type: "dir".to_string(),
            date: "2024-05-01T11:19:12.1733916+08:00".to_string(),
            create_date: "2024-05-01T11:19:12.1733916+08:00".to_string(),
            source_enabled: false,
        };

        assert_eq!(obj.id, "bbTL");
        assert_eq!(obj.name, "my_folder");
        assert_eq!(obj.object_type, "dir");
        assert_eq!(obj.size, 0);
    }

    #[test]
    fn test_policy_struct() {
        let policy = Policy {
            id: "z3hJ".to_string(),
            name: "Default storage policy".to_string(),
            policy_type: "local".to_string(),
            max_size: 0,
            file_type: None,
        };

        assert_eq!(policy.id, "z3hJ");
        assert_eq!(policy.name, "Default storage policy");
        assert_eq!(policy.policy_type, "local");
    }

    #[test]
    fn test_property_struct() {
        let property = Property {
            created_at: "2024-05-01T11:05:21.8852154+08:00".to_string(),
            updated_at: "2024-04-30T15:25:46.424+08:00".to_string(),
            policy: "Default storage policy".to_string(),
            size: 1597,
            child_folder_num: 0,
            child_file_num: 0,
            path: "".to_string(),
            query_date: "2024-05-01T11:20:07.6083989+08:00".to_string(),
        };

        assert_eq!(property.size, 1597);
        assert_eq!(property.child_folder_num, 0);
        assert_eq!(property.child_file_num, 0);
    }

    #[test]
    fn test_directory_list_struct() {
        let objects = vec![Object {
            id: "bbTL".to_string(),
            name: "my_folder".to_string(),
            path: "/".to_string(),
            thumb: false,
            size: 0,
            object_type: "dir".to_string(),
            date: "2024-05-01T11:19:12.1733916+08:00".to_string(),
            create_date: "2024-05-01T11:19:12.1733916+08:00".to_string(),
            source_enabled: false,
        }];

        let dir_list = DirectoryList {
            parent: "9zh3".to_string(),
            objects,
            policy: Policy {
                id: "z3hJ".to_string(),
                name: "Default storage policy".to_string(),
                policy_type: "local".to_string(),
                max_size: 0,
                file_type: None,
            },
        };

        assert_eq!(dir_list.parent, "9zh3");
        assert_eq!(dir_list.objects.len(), 1);
    }

    #[test]
    fn test_upload_session_struct() {
        let session = UploadSession {
            session_id: "abc123".to_string(),
            chunk_size: 1024,
            expires: 1234567890,
        };

        assert_eq!(session.session_id, "abc123");
        assert_eq!(session.chunk_size, 1024);
        assert_eq!(session.expires, 1234567890);
    }

    #[test]
    fn test_storage_info_struct() {
        let storage = StorageInfo {
            used: 1597,
            free: 1073740227,
            total: 1073741824,
        };

        assert_eq!(storage.used, 1597);
        assert_eq!(storage.free, 1073740227);
        assert_eq!(storage.total, 1073741824);
    }

    #[test]
    fn test_api_response_struct() {
        let response: ApiResponse<String> = ApiResponse {
            code: 0,
            msg: "".to_string(),
            data: Some("success".to_string()),
        };

        assert_eq!(response.code, 0);
        assert!(response.data.is_some());
    }

    #[test]
    fn test_login_request_struct() {
        let request = LoginRequest {
            user_name: "admin@cloudreve.org",
            password: "password",
            captcha_code: "",
        };

        assert_eq!(request.user_name, "admin@cloudreve.org");
        assert_eq!(request.password, "password");
    }

    #[test]
    fn test_upload_file_request_struct() {
        let request = UploadFileRequest {
            path: "/test/b",
            size: 188,
            name: "1.py",
            policy_id: "kVfW",
            last_modified: 1714481625683,
            mime_type: "",
        };

        assert_eq!(request.path, "/test/b");
        assert_eq!(request.size, 188);
        assert_eq!(request.name, "1.py");
    }

    #[test]
    fn test_share_request_struct() {
        let request = ShareRequest {
            id: "dmLxcN".to_string(),
            is_dir: false,
            password: "".to_string(),
            downloads: -1,
            expire: 86400,
            preview: true,
        };

        assert_eq!(request.id, "dmLxcN");
        assert_eq!(request.downloads, -1);
        assert_eq!(request.expire, 86400);
    }
}
