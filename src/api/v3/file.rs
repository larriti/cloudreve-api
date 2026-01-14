//! File-related API endpoints for Cloudreve API v3

use crate::api::v3::models::*;
use crate::api::v3::ApiV3Client;
use crate::Error;

impl ApiV3Client {
    pub async fn upload_file(
        &self,
        request: &UploadFileRequest<'_>,
    ) -> Result<UploadSession, Error> {
        let response: ApiResponse<UploadSession> = self.put("/file/upload", request).await?;
        match response.data {
            Some(session) => Ok(session),
            None => Err(Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }

    pub async fn complete_upload(&self, session_id: &str) -> Result<(), Error> {
        let response: ApiResponse<()> = self
            .post(
                &format!("/callback/onedrive/finish/{}", session_id),
                &serde_json::json!({}),
            )
            .await?;
        if response.code == 0 {
            Ok(())
        } else {
            Err(Error::Api {
                code: response.code,
                message: response.msg,
            })
        }
    }

    pub async fn upload_chunk(
        &self,
        session_id: &str,
        _chunk_index: u32,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        let url = self.get_url(&format!("/file/upload/{}/0", session_id));
        let mut request = self.http_client.post(&url).body(data);

        if let Some(cookie) = &self.session_cookie {
            request = request.header("Cookie", format!("cloudreve-session={}", cookie));
        }

        let response = request.send().await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::Api {
                code: -1,
                message: format!("Upload failed with status: {}", response.status()),
            })
        }
    }

    pub async fn download_file(&self, id: &str) -> Result<DownloadUrl, Error> {
        let response: ApiResponse<DownloadUrl> = self
            .put(&format!("/file/download/{}", id), &serde_json::json!({}))
            .await?;
        match response.data {
            Some(url) => Ok(url),
            None => Err(Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }

    pub async fn get_file_source(
        &self,
        request: &FileSourceRequest,
    ) -> Result<Vec<FileSource>, Error> {
        let response: ApiResponse<Vec<FileSource>> = self.post("/file/source", request).await?;
        match response.data {
            Some(sources) => Ok(sources),
            None => Err(Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }

    pub async fn preview_file(&self, id: &str) -> Result<DirectoryList, Error> {
        let response: ApiResponse<DirectoryList> =
            self.get(&format!("/file/preview/{}", id)).await?;
        match response.data {
            Some(list) => Ok(list),
            None => Err(Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }

    pub async fn get_thumbnail(&self, id: &str) -> Result<DirectoryList, Error> {
        let response: ApiResponse<DirectoryList> = self.get(&format!("/file/thumb/{}", id)).await?;
        match response.data {
            Some(list) => Ok(list),
            None => Err(Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }

    pub async fn create_file(&self, request: &CreateFileRequest<'_>) -> Result<(), Error> {
        let response: ApiResponse<()> = self.post("/file/create", request).await?;
        if response.code == 0 {
            Ok(())
        } else {
            Err(Error::Api {
                code: response.code,
                message: response.msg,
            })
        }
    }
}
