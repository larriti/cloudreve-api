//! Object-related API endpoints for Cloudreve API v3

use crate::api::v3::models::*;
use crate::api::v3::ApiV3Client;
use crate::Error;

impl ApiV3Client {
    /// Get object (file/folder) property
    pub async fn get_object_property(
        &self,
        id: &str,
        is_folder: Option<bool>,
        trace_root: Option<bool>,
    ) -> Result<Property, Error> {
        let mut query_params = Vec::new();
        if let Some(folder) = is_folder {
            query_params.push(format!("is_folder={}", folder));
        }
        if let Some(trace) = trace_root {
            query_params.push(format!("trace_root={}", trace));
        }

        let endpoint = if query_params.is_empty() {
            format!("/object/property/{}", id)
        } else {
            format!("/object/property/{}?{}", id, query_params.join("&"))
        };

        let response: ApiResponse<Property> = self.get(&endpoint).await?;
        match response.data {
            Some(property) => Ok(property),
            None => Err(Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }

    /// Rename object
    pub async fn rename_object(&self, request: &RenameObjectRequest<'_>) -> Result<(), Error> {
        let response: ApiResponse<()> = self.post("/object/rename", request).await?;
        if response.code == 0 {
            Ok(())
        } else {
            Err(Error::Api {
                code: response.code,
                message: response.msg,
            })
        }
    }

    /// Move object
    pub async fn move_object(&self, request: &MoveObjectRequest<'_>) -> Result<(), Error> {
        let response: ApiResponse<()> = self.patch("/object", request).await?;
        if response.code == 0 {
            Ok(())
        } else {
            Err(Error::Api {
                code: response.code,
                message: response.msg,
            })
        }
    }

    /// Copy object
    pub async fn copy_object(&self, request: &CopyObjectRequest<'_>) -> Result<(), Error> {
        let response: ApiResponse<()> = self.post("/object/copy", request).await?;
        if response.code == 0 {
            Ok(())
        } else {
            Err(Error::Api {
                code: response.code,
                message: response.msg,
            })
        }
    }

    /// Delete object
    pub async fn delete_object(&self, request: &DeleteObjectRequest<'_>) -> Result<(), Error> {
        let response: ApiResponse<()> = self.delete_with_body("/object", request).await?;
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
