//! Workflow-related API endpoints for Cloudreve v4 API

use crate::api::v4::models::*;
use crate::api::v4::ApiV4Client;
use crate::Error;

impl ApiV4Client {
    pub async fn create_remote_download(
        &self,
        request: &CreateRemoteDownloadRequest<'_>,
    ) -> Result<Task, Error> {
        let response: ApiResponse<Task> = self.post("/workflow/remote-download", request).await?;
        Ok(response.data.unwrap())
    }

    pub async fn create_download(
        &self,
        request: &CreateDownloadRequest<'_>,
    ) -> Result<Task, Error> {
        let response: ApiResponse<Task> = self.post("/workflow/download", request).await?;
        Ok(response.data.unwrap())
    }

    pub async fn select_download_files(
        &self,
        task_id: &str,
        request: &SelectDownloadFilesRequest<'_>,
    ) -> Result<Task, Error> {
        let response: ApiResponse<Task> = self
            .patch(&format!("/workflow/download/{}", task_id), request)
            .await?;
        Ok(response.data.unwrap())
    }

    pub async fn cancel_download_task(&self, task_id: &str) -> Result<Task, Error> {
        let response: ApiResponse<Task> = self
            .delete(&format!("/workflow/download/{}", task_id))
            .await?;
        Ok(response.data.unwrap())
    }

    pub async fn list_workflow_tasks(&self) -> Result<TaskListResponse, Error> {
        let response: ApiResponse<TaskListResponse> = self.get("/workflow").await?;
        Ok(response.data.unwrap())
    }

    pub async fn get_task_progress(&self, task_id: &str) -> Result<Progress, Error> {
        let response: ApiResponse<Progress> =
            self.get(&format!("/workflow/progress/{}", task_id)).await?;
        Ok(response.data.unwrap())
    }

    pub async fn cancel_task(&self, task_id: &str) -> Result<Task, Error> {
        let response: ApiResponse<Task> =
            self.delete(&format!("/workflow/tasks/{}", task_id)).await?;
        Ok(response.data.unwrap())
    }

    pub async fn list_tasks(&self, request: &ListTasksRequest<'_>) -> Result<Vec<Task>, Error> {
        let mut url = "/workflow/tasks".to_string();
        if let Some(page) = request.page {
            url.push_str(&format!("?page={}", page));
        }
        if let Some(per_page) = request.per_page {
            url.push_str(&format!("&per_page={}", per_page));
        }
        if let Some(status) = request.status {
            url.push_str(&format!("&status={}", status));
        }
        if let Some(type_) = request.type_ {
            url.push_str(&format!("&type={}", type_));
        }

        let response: ApiResponse<Vec<Task>> = self.get(&url).await?;
        Ok(response.data.unwrap())
    }

    pub async fn create_archive(&self, request: &CreateArchiveRequest<'_>) -> Result<Task, Error> {
        let response: ApiResponse<Task> = self.post("/workflow/archive", request).await?;
        Ok(response.data.unwrap())
    }

    pub async fn extract_archive(
        &self,
        request: &ExtractArchiveRequest<'_>,
    ) -> Result<Task, Error> {
        let response: ApiResponse<Task> = self.post("/workflow/extract", request).await?;
        Ok(response.data.unwrap())
    }

    pub async fn relocate(&self, request: &RelocateRequest<'_>) -> Result<Task, Error> {
        let response: ApiResponse<Task> = self.post("/workflow/relocate", request).await?;
        Ok(response.data.unwrap())
    }

    pub async fn import(&self, request: &ImportRequest<'_>) -> Result<Task, Error> {
        let response: ApiResponse<Task> = self.post("/workflow/import", request).await?;
        Ok(response.data.unwrap())
    }
}
