use cloudreve_api::api::v3::models::*;
use cloudreve_api::{ApiV3Client, Result};
use tokio;

#[cfg(test)]
mod v3_file_tests {
    use super::*;

    #[tokio::test]
    async fn test_upload_request_struct() -> Result<()> {
        let _upload_request = UploadFileRequest {
            path: "/uploads",
            size: 1024,
            name: "test.txt",
            policy_id: "policy123",
            last_modified: 1234567890,
            mime_type: "text/plain",
        };
        Ok(())
    }

    #[tokio::test]
    async fn test_create_file_request_struct() -> Result<()> {
        let _create_request = CreateFileRequest {
            path: "/new/file.txt",
        };
        Ok(())
    }

    #[tokio::test]
    async fn test_file_source_request_struct() -> Result<()> {
        let _source_request = FileSourceRequest {
            items: vec!["file1".to_string(), "file2".to_string()],
        };
        Ok(())
    }

    #[test]
    fn test_upload_session_struct() {
        let session = UploadSession {
            session_id: "session123".to_string(),
            chunk_size: 5242880,
            expires: 3600,
            upload_urls: vec!["https://example.com/upload/0".to_string()],
        };

        assert_eq!(session.session_id, "session123");
        assert_eq!(session.chunk_size, 5242880);
        assert_eq!(session.upload_urls.len(), 1);
    }

    #[test]
    fn test_file_source_struct() {
        let source = FileSource {
            url: "https://example.com/f/DLCZ/97-1.txt".to_string(),
            name: "97-1.txt".to_string(),
            parent: 1110,
        };

        assert_eq!(source.url, "https://example.com/f/DLCZ/97-1.txt");
        assert_eq!(source.name, "97-1.txt");
        assert_eq!(source.parent, 1110);
    }

    #[test]
    fn test_directory_list_struct() {
        let objects = vec![Object {
            id: "j6hJ".to_string(),
            name: "test.txt".to_string(),
            path: "/".to_string(),
            thumb: false,
            size: 1597,
            object_type: "file".to_string(),
            date: "2024-04-30T15:25:46.424+08:00".to_string(),
            create_date: "2024-05-01T11:05:21.8852154+08:00".to_string(),
            source_enabled: false,
        }];

        let list = DirectoryList {
            parent: "9zh3".to_string(),
            objects,
            policy: Policy {
                id: "z3hJ".to_string(),
                name: "Default storage policy".to_string(),
                policy_type: "local".to_string(),
                max_size: 0,
                file_type: vec![],
            },
        };

        assert_eq!(list.parent, "9zh3");
        assert_eq!(list.objects.len(), 1);
        assert_eq!(list.objects[0].name, "test.txt");
    }
}
