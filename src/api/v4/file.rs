//! File-related API endpoints for Cloudreve v4 API

use crate::Error;
use crate::api::v4::ApiV4Client;
use crate::api::v4::models::*;
use crate::api::v4::uri::*;

/// File management methods
impl ApiV4Client {
    pub async fn upload_file(&self, request: &UploadRequest<'_>) -> Result<File, Error> {
        let response: ApiResponse<File> = self.post("/file", request).await?;
        match response.data {
            Some(data) => Ok(data),
            None => Err(Error::InvalidResponse(format!(
                "API returned no data for upload_file request: {:?}",
                response
            ))),
        }
    }

    /// Lists files in a directory with full response including metadata
    ///
    /// Returns a complete `ListResponse` containing:
    /// - `files`: Vector of files/folders in the directory
    /// - `parent`: Information about the parent directory
    /// - `pagination`: Pagination information (page, total_items, etc.)
    /// - `storage_policy`: Preferred storage policy for uploads to this directory
    /// - `props`: Navigator capabilities and settings
    ///
    /// # Arguments
    /// * `request` - ListFilesRequest with path and optional pagination params
    pub async fn list_files(&self, request: &ListFilesRequest<'_>) -> Result<ListResponse, Error> {
        let mut url = "/file".to_string();
        if let Some(path) = request.path.strip_prefix('/') {
            url.push_str(&format!("?uri={}", path_to_uri(path)));
        } else {
            url.push_str(&format!("?uri={}", path_to_uri(request.path)));
        }
        if let Some(page) = request.page {
            url.push_str(&format!("&page={}", page));
        }
        if let Some(page_size) = request.page_size {
            url.push_str(&format!("&page_size={}", page_size));
        }
        if let Some(order_by) = request.order_by {
            url.push_str(&format!("&order_by={}", order_by));
        }
        if let Some(order_direction) = request.order_direction {
            url.push_str(&format!("&order_direction={}", order_direction));
        }
        if let Some(next_page_token) = request.next_page_token {
            url.push_str(&format!("&next_page_token={}", next_page_token));
        }

        let response: ApiResponse<ListResponse> = self.get(&url).await?;
        match response.data {
            Some(data) => Ok(data),
            None => Err(Error::InvalidResponse(format!(
                "API returned no data for list_files request: {:?}",
                response
            ))),
        }
    }

    pub async fn get_file_info(&self, file_path: &str) -> Result<File, Error> {
        // URI encode the path for V4 API, use /file/info endpoint
        let uri = path_to_uri(file_path);
        let response: ApiResponse<File> = self.get(&format!("/file/info?uri={}", uri)).await?;
        match response.data {
            Some(data) => Ok(data),
            None => Err(Error::InvalidResponse(format!(
                "API returned no data for get_file_info request: {:?}",
                response
            ))),
        }
    }

    pub async fn get_file_stat(&self, file_path: &str) -> Result<FileStat, Error> {
        let response: ApiResponse<FileStat> =
            self.get(&format!("/file/stat/{}", file_path)).await?;
        match response.data {
            Some(data) => Ok(data),
            None => Err(Error::InvalidResponse(format!(
                "API returned no data for get_file_stat request: {:?}",
                response
            ))),
        }
    }

    pub async fn move_file(&self, request: &MoveFileRequest<'_>) -> Result<(), Error> {
        let _: ApiResponse<()> = self.post("/file/move", request).await?;
        Ok(())
    }

    pub async fn copy_file(&self, request: &CopyFileRequest<'_>) -> Result<(), Error> {
        let _: ApiResponse<()> = self.post("/file/copy", request).await?;
        Ok(())
    }

    pub async fn rename_file(
        &self,
        file_path: &str,
        request: &RenameFileRequest<'_>,
    ) -> Result<(), Error> {
        // V4 API may not have /file/rename endpoint, use /file/move instead
        // Extract parent directory and construct new path
        let uri = path_to_uri(file_path);
        let new_uri = if let Some(parent) = file_path.rsplit('/').skip(1).next() {
            if parent.is_empty() {
                // Root directory
                path_to_uri(&format!("/{}", request.name))
            } else {
                path_to_uri(&format!("{}/{}", parent, request.name))
            }
        } else {
            path_to_uri(&format!("/{}", request.name))
        };

        let move_req = MoveFileRequest {
            from: &uri,
            to: &new_uri,
        };
        self.move_file(&move_req).await
    }

    pub async fn delete_file(&self, file_path: &str) -> Result<(), Error> {
        // URI encode the path for V4 API
        let uri = path_to_uri(file_path);
        let url = format!("/file?uri={}", uri);
        let _: ApiResponse<()> = self.delete(&url).await?;
        Ok(())
    }

    pub async fn create_directory(&self, path: &str) -> Result<(), Error> {
        // Convert path to URI format
        let uri = path_to_uri(path);

        // Use the correct /file/create endpoint
        let request = serde_json::json!({
            "uri": uri,
            "type": "folder"
        });

        let response: ApiResponse<serde_json::Value> = self.post("/file/create", &request).await?;
        match response.code {
            0 => Ok(()),
            code => Err(Error::Api {
                code,
                message: response.msg,
            }),
        }
    }

    pub async fn set_file_permission(
        &self,
        request: &SetFilePermissionRequest<'_>,
    ) -> Result<(), Error> {
        let _: ApiResponse<()> = self.post("/file/permission", request).await?;
        Ok(())
    }

    pub async fn delete_file_permission(&self, path: &str) -> Result<(), Error> {
        let uri = path_to_uri(path);
        let _: ApiResponse<()> = self
            .delete(&format!("/file/permission?uri={}", uri))
            .await?;
        Ok(())
    }

    pub async fn create_upload_session(
        &self,
        request: &CreateUploadSessionRequest<'_>,
    ) -> Result<UploadSessionResponse, Error> {
        let response: ApiResponse<UploadSessionResponse> =
            self.put("/file/upload", request).await?;
        match response.data {
            Some(data) => Ok(data),
            None => Err(Error::InvalidResponse(format!(
                "API returned no data for create_upload_session request: {:?}",
                response
            ))),
        }
    }

    pub async fn upload_file_chunk(
        &self,
        session_id: &str,
        index: u32,
        chunk_data: &[u8],
    ) -> Result<(), Error> {
        let url = format!("/file/upload/{}/{}", session_id, index);
        let full_url = self.get_url(&url);

        let mut request = self.http_client.post(&full_url).body(chunk_data.to_vec());

        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }

        let response = request.send().await?;
        let status = response.status();

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(Error::Api {
                code: status.as_u16() as i32,
                message: error_text,
            });
        }

        Ok(())
    }

    pub async fn delete_upload_session(&self, path: &str, session_id: &str) -> Result<(), Error> {
        let uri = path_to_uri(path);
        let request = DeleteUploadSessionRequest {
            id: session_id,
            uri: &uri,
        };
        let url = self.get_url("/file/upload");

        let body = serde_json::to_string(&request)?;

        let mut http_req = self.http_client.delete(&url);
        if let Some(token) = &self.token {
            http_req = http_req.bearer_auth(token);
        }

        let response = http_req
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(Error::Api {
                code: status.as_u16() as i32,
                message: error_text,
            });
        }

        Ok(())
    }

    pub async fn get_thumbnail_url(
        &self,
        path: &str,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<String, Error> {
        let uri = path_to_uri(path);
        let mut url = format!("/file/thumb?uri={}", uri);
        if let Some(w) = width {
            url.push_str(&format!("&width={}", w));
        }
        if let Some(h) = height {
            url.push_str(&format!("&height={}", h));
        }

        let response: ApiResponse<String> = self.get(&url).await?;
        match response.data {
            Some(data) => Ok(data),
            None => Err(Error::InvalidResponse(format!(
                "API returned no data for get_thumbnail_url request: {:?}",
                response
            ))),
        }
    }

    pub async fn get_file_content(&self, path: &str) -> Result<String, Error> {
        let uri = path_to_uri(path);
        let response: ApiResponse<String> = self.get(&format!("/file/content?uri={}", uri)).await?;
        match response.data {
            Some(data) => Ok(data),
            None => Err(Error::InvalidResponse(format!(
                "API returned no data for get_file_content request: {:?}",
                response
            ))),
        }
    }

    pub async fn update_file_content(
        &self,
        request: &UpdateFileContentRequest<'_>,
    ) -> Result<(), Error> {
        let _: ApiResponse<()> = self.put("/file/content", request).await?;
        Ok(())
    }

    pub async fn create_viewer_session(
        &self,
        request: &CreateViewerSessionRequest<'_>,
    ) -> Result<ViewerSessionResponse, Error> {
        let response: ApiResponse<ViewerSessionResponse> =
            self.put("/file/viewerSession", request).await?;
        match response.data {
            Some(data) => Ok(data),
            None => Err(Error::InvalidResponse(format!(
                "API returned no data for create_viewer_session request: {:?}",
                response
            ))),
        }
    }

    pub async fn create_file(&self, request: &CreateFileRequest<'_>) -> Result<File, Error> {
        let response: ApiResponse<File> = self.post("/file/create", request).await?;
        match response.data {
            Some(data) => Ok(data),
            None => Err(Error::InvalidResponse(format!(
                "API returned no data for create_file request: {:?}",
                response
            ))),
        }
    }

    pub async fn rename_multiple(&self, request: &RenameMultipleRequest<'_>) -> Result<(), Error> {
        let _: ApiResponse<()> = self.post("/file/rename", request).await?;
        Ok(())
    }

    pub async fn move_copy_files(&self, request: &MoveCopyFileRequest<'_>) -> Result<(), Error> {
        let _: ApiResponse<()> = self.post("/file/move", request).await?;
        Ok(())
    }

    pub async fn create_download_url(
        &self,
        request: &CreateDownloadUrlRequest<'_>,
    ) -> Result<DownloadUrlResponse, Error> {
        let uris = paths_to_uris(&request.uris);
        let uris_refs: Vec<&str> = uris.iter().map(|s| s.as_str()).collect();

        let converted_request = CreateDownloadUrlRequest {
            uris: uris_refs,
            download: request.download,
            redirect: request.redirect,
            entity: request.entity,
            use_primary_site_url: request.use_primary_site_url,
            skip_error: request.skip_error,
            archive: request.archive,
            no_cache: request.no_cache,
        };

        let response: ApiResponse<DownloadUrlResponse> =
            self.post("/file/url", &converted_request).await?;
        match response.data {
            Some(data) => Ok(data),
            None => Err(Error::InvalidResponse(format!(
                "API returned no data for create_download_url request: {:?}",
                response
            ))),
        }
    }

    pub async fn restore_from_trash(&self, request: &RestoreFileRequest<'_>) -> Result<(), Error> {
        let uris = paths_to_uris(&request.uris);
        let uris_refs: Vec<&str> = uris.iter().map(|s| s.as_str()).collect();

        let converted_request = RestoreFileRequest { uris: uris_refs };

        let _: ApiResponse<()> = self.post("/file/restore", &converted_request).await?;
        Ok(())
    }

    pub async fn force_unlock(&self, path: &str) -> Result<(), Error> {
        let uri = path_to_uri(path);
        let _: ApiResponse<()> = self.delete(&format!("/file/lock?uri={}", uri)).await?;
        Ok(())
    }

    pub async fn patch_metadata(
        &self,
        path: &str,
        request: &UpdateMetadataRequest,
    ) -> Result<(), Error> {
        let uri = path_to_uri(path);
        let full_url = format!("/file/metadata?uri={}", uri);
        let _: ApiResponse<()> = self.patch(&full_url, request).await?;
        Ok(())
    }

    pub async fn mount_storage_policy(
        &self,
        path: &str,
        request: &MountStoragePolicyRequest,
    ) -> Result<(), Error> {
        let uri = path_to_uri(path);
        let full_url = format!("/file/policy?uri={}", uri);
        let _: ApiResponse<()> = self.patch(&full_url, request).await?;
        Ok(())
    }

    pub async fn update_view_settings(
        &self,
        path: &str,
        request: &UpdateViewRequest,
    ) -> Result<(), Error> {
        let uri = path_to_uri(path);
        let full_url = format!("/file/view?uri={}", uri);
        let _: ApiResponse<()> = self.patch(&full_url, request).await?;
        Ok(())
    }

    pub async fn get_file_activities(
        &self,
        path: &str,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<FileActivitiesResponse, Error> {
        let uri = path_to_uri(path);
        let mut url = format!("/file/activities?uri={}", uri);
        if let Some(p) = page {
            url.push_str(&format!("&page={}", p));
        }
        if let Some(ps) = page_size {
            url.push_str(&format!("&page_size={}", ps));
        }

        let response: ApiResponse<FileActivitiesResponse> = self.get(&url).await?;
        match response.data {
            Some(data) => Ok(data),
            None => Err(Error::InvalidResponse(format!(
                "API returned no data for get_file_activities request: {:?}",
                response
            ))),
        }
    }

    pub async fn get_file_info_extended(
        &self,
        request: &GetFileInfoRequest<'_>,
    ) -> Result<File, Error> {
        let uri = path_to_uri(request.uri);
        let mut url = format!("/file/info?uri={}", uri);
        if let Some(include_extended) = request.include_extended_info {
            url.push_str(&format!("&extended={}", include_extended));
        }

        let response: ApiResponse<File> = self.get(&url).await?;
        match response.data {
            Some(data) => Ok(data),
            None => Err(Error::InvalidResponse(format!(
                "API returned no data for get_file_info_extended request: {:?}",
                response
            ))),
        }
    }

    pub async fn get_archive_list(
        &self,
        request: &GetArchiveListRequest<'_>,
    ) -> Result<ArchiveListResponse, Error> {
        let uri = path_to_uri(request.uri);
        let url = format!("/file/archive?uri={}", uri);

        let response: ApiResponse<ArchiveListResponse> = self.get(&url).await?;
        match response.data {
            Some(data) => Ok(data),
            None => Err(Error::InvalidResponse(format!(
                "API returned no data for get_archive_list request: {:?}",
                response
            ))),
        }
    }
}
