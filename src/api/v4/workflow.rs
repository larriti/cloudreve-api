//! Workflow-related API endpoints for Cloudreve v4 API

use crate::Error;
use crate::api::v4::ApiV4Client;
use crate::api::v4::models::*;

impl ApiV4Client {
    pub async fn create_download(
        &self,
        request: &CreateDownloadRequest<'_>,
    ) -> Result<Vec<Task>, Error> {
        let response: ApiResponse<Vec<Task>> = self.post("/workflow/download", request).await?;
        if response.code != 0 {
            return Err(Error::Api {
                code: response.code,
                message: response.msg,
            });
        }
        response
            .data
            .ok_or_else(|| Error::InvalidResponse("Missing data in API response".to_string()))
    }

    pub async fn select_download_files(
        &self,
        task_id: &str,
        request: &SelectDownloadFilesRequest<'_>,
    ) -> Result<Vec<Task>, Error> {
        let response: ApiResponse<Vec<Task>> = self
            .patch(&format!("/workflow/download/{}", task_id), request)
            .await?;
        if response.code != 0 {
            return Err(Error::Api {
                code: response.code,
                message: response.msg,
            });
        }
        response
            .data
            .ok_or_else(|| Error::InvalidResponse("Missing data in API response".to_string()))
    }

    pub async fn cancel_download_task(&self, task_id: &str) -> Result<(), Error> {
        // API returns only code and msg, no data field
        #[derive(Debug, serde::Deserialize)]
        struct EmptyResponse;
        let response: ApiResponse<EmptyResponse> = self
            .delete(&format!("/workflow/download/{}", task_id))
            .await?;
        if response.code != 0 {
            return Err(Error::Api {
                code: response.code,
                message: response.msg,
            });
        }
        Ok(())
    }

    pub async fn list_workflow_tasks(
        &self,
        page_size: i32,
        category: &str,
    ) -> Result<TaskListResponse, Error> {
        let url = format!("/workflow?page_size={}&category={}", page_size, category);
        let response: ApiResponse<TaskListResponse> = self.get(&url).await?;
        if response.code != 0 {
            return Err(Error::Api {
                code: response.code,
                message: response.msg,
            });
        }
        response
            .data
            .ok_or_else(|| Error::InvalidResponse("Missing data in API response".to_string()))
    }

    pub async fn get_task_progress(&self, task_id: &str) -> Result<Progress, Error> {
        let response: ApiResponse<Progress> =
            self.get(&format!("/workflow/progress/{}", task_id)).await?;
        if response.code != 0 {
            return Err(Error::Api {
                code: response.code,
                message: response.msg,
            });
        }
        response
            .data
            .ok_or_else(|| Error::InvalidResponse("Missing data in API response".to_string()))
    }

    pub async fn create_archive(
        &self,
        request: &CreateArchiveRequest<'_>,
    ) -> Result<TaskResponse, Error> {
        let response: ApiResponse<TaskResponse> = self.post("/workflow/archive", request).await?;
        if response.code != 0 {
            return Err(Error::Api {
                code: response.code,
                message: response.msg,
            });
        }
        response
            .data
            .ok_or_else(|| Error::InvalidResponse("Missing data in API response".to_string()))
    }

    pub async fn extract_archive(
        &self,
        request: &ExtractArchiveRequest<'_>,
    ) -> Result<TaskResponse, Error> {
        let response: ApiResponse<TaskResponse> = self.post("/workflow/extract", request).await?;
        if response.code != 0 {
            return Err(Error::Api {
                code: response.code,
                message: response.msg,
            });
        }
        response
            .data
            .ok_or_else(|| Error::InvalidResponse("Missing data in API response".to_string()))
    }

    pub async fn relocate(&self, request: &RelocateRequest<'_>) -> Result<TaskResponse, Error> {
        let response: ApiResponse<TaskResponse> = self.post("/workflow/relocate", request).await?;
        if response.code != 0 {
            return Err(Error::Api {
                code: response.code,
                message: response.msg,
            });
        }
        response
            .data
            .ok_or_else(|| Error::InvalidResponse("Missing data in API response".to_string()))
    }

    pub async fn import(&self, request: &ImportRequest<'_>) -> Result<TaskResponse, Error> {
        let response: ApiResponse<TaskResponse> = self.post("/workflow/import", request).await?;
        if response.code != 0 {
            return Err(Error::Api {
                code: response.code,
                message: response.msg,
            });
        }
        response
            .data
            .ok_or_else(|| Error::InvalidResponse("Missing data in API response".to_string()))
    }
}
