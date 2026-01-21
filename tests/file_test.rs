use cloudreve_api::api::v4::models::*;
use cloudreve_api::{DeleteResult, Result};

#[cfg(test)]
mod file_tests {
    use super::*;

    #[test]
    fn test_delete_result_struct() {
        let result = DeleteResult::default();
        assert_eq!(result.deleted, 0);
        assert_eq!(result.failed, 0);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_delete_result_with_values() {
        let mut result = DeleteResult {
            deleted: 5,
            failed: 1,
            ..Default::default()
        };
        result
            .errors
            .push(("path1".to_string(), "error1".to_string()));

        assert_eq!(result.deleted, 5);
        assert_eq!(result.failed, 1);
        assert_eq!(result.errors.len(), 1);
    }

    #[tokio::test]
    async fn test_upload_request_struct() -> Result<()> {
        let _upload_request = UploadRequest {
            path: "/uploads",
            name: Some("test.txt"),
            overwrite: Some(false),
        };
        Ok(())
    }

    #[tokio::test]
    async fn test_list_files_request_struct() -> Result<()> {
        let _list_request = ListFilesRequest {
            path: "/",
            page: Some(1),
            page_size: Some(20),
            order_by: Some("name"),
            order_direction: Some("asc"),
            next_page_token: None,
        };
        Ok(())
    }

    #[tokio::test]
    async fn test_move_file_request_struct() -> Result<()> {
        let _move_request = MoveFileRequest {
            uris: vec!["/old/path"],
            dst: "/new/path",
            copy: None,
        };
        Ok(())
    }

    #[tokio::test]
    async fn test_copy_file_request_struct() -> Result<()> {
        let _copy_request = CopyFileRequest {
            uris: vec!["/source/path"],
            dst: "/dest/path",
        };
        Ok(())
    }

    #[tokio::test]
    async fn test_rename_file_request_struct() -> Result<()> {
        let _rename_request = RenameFileRequest {
            uri: "/old/path",
            new_name: "new_name.txt",
        };
        Ok(())
    }

    #[test]
    fn test_file_structs() {
        let _file = File {
            r#type: FileType::File,
            id: "1".to_string(),
            name: "test.txt".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
            updated_at: "2023-01-01T00:00:00Z".to_string(),
            size: 1024,
            metadata: None,
            path: "/path/test.txt".to_string(),
            capability: Some("read".to_string()),
            owned: true,
            primary_entity: Some("primary".to_string()),
            permission: Some("read".to_string()),
        };

        let _file_stat = FileStat {
            size: 1024,
            created_at: "2023-01-01T00:00:00Z".to_string(),
            updated_at: "2023-01-01T00:00:00Z".to_string(),
            mime_type: "text/plain".to_string(),
        };

        let _list_response = ListResponse {
            files: vec![_file.clone()],
            parent: _file.clone(),
            pagination: PaginationResults {
                page: 1,
                page_size: 20,
                total_items: Some(100),
                next_token: Some("".to_string()),
                is_cursor: false,
            },
            props: NavigatorProps {
                capability: "read".to_string(),
                max_page_size: 100,
                order_by_options: vec!["name".to_string(), "size".to_string()],
                order_direction_options: vec!["asc".to_string(), "desc".to_string()],
            },
            context_hint: "files".to_string(),
            mixed_type: true,
            storage_policy: None,
            view: None,
        };
    }
}

#[test]
fn test_list_response_parsing() {
    use cloudreve_api::api::v4::models::*;

    let json = r#"{
        "code": 0,
        "data": {
            "files": [
                {"type": 1, "id": "1", "name": "folder", "created_at": "2023-01-01T00:00:00Z", "updated_at": "2023-01-01T00:00:00Z", "size": 0, "metadata": null, "path": "cloudreve://my/folder", "capability": "test", "owned": true, "primary_entity": "test"},
                {"type": 0, "id": "2", "name": "file.txt", "created_at": "2023-01-01T00:00:00Z", "updated_at": "2023-01-01T00:00:00Z", "size": 100, "metadata": {}, "path": "cloudreve://my/file.txt", "capability": "test", "owned": true, "primary_entity": "test"}
            ],
            "parent": {"type": 1, "id": "0", "name": "", "created_at": "2023-01-01T00:00:00Z", "updated_at": "2023-01-01T00:00:00Z", "size": 0, "metadata": null, "path": "cloudreve://my/", "capability": "test", "owned": true, "primary_entity": "test"},
            "pagination": {"page": 0, "page_size": 50, "is_cursor": true},
            "props": {"capability": "test", "max_page_size": 2000, "order_by_options": ["name"], "order_direction_options": ["asc", "desc"]},
            "context_hint": "test-uuid",
            "mixed_type": false,
            "storage_policy": {"id": "test", "name": "test", "type": "local", "max_size": 0},
            "view": {"page_size": 50, "order": "created_at", "order_direction": "asc", "view": "grid", "thumbnail": true}
        },
        "msg": ""
    }"#;

    let response: ApiResponse<ListResponse> = serde_json::from_str(json).unwrap();
    assert_eq!(response.code, 0);
    assert!(response.data.is_some());
    let data = response.data.unwrap();
    assert_eq!(data.files.len(), 2);
    assert_eq!(data.files[0].r#type, FileType::Folder);
    assert_eq!(data.files[1].r#type, FileType::File);
}
