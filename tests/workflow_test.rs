use cloudreve_api::Result;
use cloudreve_api::api::v4::models::*;

#[cfg(test)]
mod workflow_tests {
    use super::*;

    #[tokio::test]
    async fn test_create_download_request_struct() -> Result<()> {
        let _download_request = CreateDownloadRequest {
            dst: "/downloads",
            src: vec!["https://example.com/file.zip"],
            preferred_node_id: None,
        };
        Ok(())
    }

    #[tokio::test]
    async fn test_list_tasks_request_struct() -> Result<()> {
        let _list_request = ListTasksRequest {
            page: Some(1),
            per_page: Some(20),
            status: Some("running"),
            type_: Some("download"),
        };
        Ok(())
    }

    #[tokio::test]
    async fn test_create_archive_request_struct() -> Result<()> {
        let _archive_request = CreateArchiveRequest {
            src: vec!["/path/file1.txt", "/path/file2.txt"],
            dst: "/archives/archive.zip",
        };
        Ok(())
    }

    #[tokio::test]
    async fn test_extract_archive_request_struct() -> Result<()> {
        let _extract_request = ExtractArchiveRequest {
            src: vec!["/path/archive.zip"],
            dst: "/extracted",
        };
        Ok(())
    }

    #[test]
    fn test_workflow_structs() {
        let _task = Task {
            id: "task1".to_string(),
            name: Some("Download Task".to_string()),
            status: "running".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
            updated_at: "2023-01-01T00:00:00Z".to_string(),
        };

        let _task_progress = TaskProgress {
            progress: 50.0,
            message: "Downloading...".to_string(),
            total: Some(100),
            current: Some(50),
        };

        let _detailed_task = DetailedTask {
            id: "task1".to_string(),
            name: "Download Task".to_string(),
            status: "running".to_string(),
            type_: "download".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
            updated_at: "2023-01-01T00:00:00Z".to_string(),
            progress: Some(_task_progress.clone()),
        };

        let _progress = Progress {
            total: Some(100),
            current: Some(50),
            identifier: Some("task1".to_string()),
        };

        let _task_pagination = TaskPagination {
            page_size: 20,
            next_token: Some("".to_string()),
            is_cursor: false,
        };

        // Correct: TaskListResponse expects TaskResponse in the tasks vector
        let _task_list_response = TaskListResponse {
            pagination: _task_pagination.clone(),
            tasks: vec![], // Using empty vector since TaskResponse is different from DetailedTask
        };
    }
}
