//! User-related API endpoints for Cloudreve v4 API

use crate::Error;
use crate::api::v4::ApiV4Client;
use crate::api::v4::models::*;

impl ApiV4Client {
    pub async fn register(&self, request: &RegisterRequest<'_>) -> Result<User, Error> {
        let response: ApiResponse<User> = self.post("/user/register", request).await?;
        Ok(response.data.unwrap())
    }

    pub async fn get_user_capacity(&self) -> Result<Quota, Error> {
        let response: ApiResponse<Quota> = self.get("/user/capacity").await?;
        Ok(response.data.unwrap())
    }

    pub async fn search_users(&self, request: &SearchUserRequest<'_>) -> Result<Vec<User>, Error> {
        let endpoint = format!("/user/search?q={}", request.query);
        let mut query_params = Vec::new();
        if let Some(page) = request.page {
            query_params.push(format!("page={}", page));
        }
        if let Some(page_size) = request.page_size {
            query_params.push(format!("page_size={}", page_size));
        }

        let full_endpoint = if !query_params.is_empty() {
            format!("{}&{}", endpoint, query_params.join("&"))
        } else {
            endpoint
        };

        let response: ApiResponse<Vec<User>> = self.get(&full_endpoint).await?;
        Ok(response.data.unwrap())
    }

    pub async fn get_credit_changes(
        &self,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<Vec<CreditChangeRecord>, Error> {
        let mut endpoint = "/user/creditChanges".to_string();
        let mut query_params = Vec::new();
        if let Some(p) = page {
            query_params.push(format!("page={}", p));
        }
        if let Some(ps) = page_size {
            query_params.push(format!("page_size={}", ps));
        }

        if !query_params.is_empty() {
            endpoint = format!("{}?{}", endpoint, query_params.join("&"));
        }

        let response: ApiResponse<Vec<CreditChangeRecord>> = self.get(&endpoint).await?;
        Ok(response.data.unwrap())
    }

    pub async fn get_payment_records(
        &self,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<Vec<PaymentRecord>, Error> {
        let mut endpoint = "/user/payments".to_string();
        let mut query_params = Vec::new();
        if let Some(p) = page {
            query_params.push(format!("page={}", p));
        }
        if let Some(ps) = page_size {
            query_params.push(format!("page_size={}", ps));
        }

        if !query_params.is_empty() {
            endpoint = format!("{}?{}", endpoint, query_params.join("&"));
        }

        let response: ApiResponse<Vec<PaymentRecord>> = self.get(&endpoint).await?;
        Ok(response.data.unwrap())
    }

    pub async fn update_user_setting(
        &self,
        setting: &UpdateUserSettingRequest<'_>,
    ) -> Result<(), Error> {
        let response: ApiResponse<()> = self.patch("/user/setting", setting).await?;
        if response.code == 0 {
            Ok(())
        } else {
            Err(Error::Api {
                code: response.code,
                message: response.msg,
            })
        }
    }

    pub async fn get_user_setting(&self) -> Result<UserSettings, Error> {
        let response: ApiResponse<UserSettings> = self.get("/user/setting").await?;
        Ok(response.data.unwrap())
    }

    pub async fn enable_two_factor(&self) -> Result<TwoFactorSetup, Error> {
        let response: ApiResponse<TwoFactorSetup> = self.post("/user/2fa/enable", &()).await?;
        Ok(response.data.unwrap())
    }

    pub async fn verify_two_factor(&self, request: &TwoFactorVerify) -> Result<(), Error> {
        let response: ApiResponse<()> = self.post("/user/2fa/verify", request).await?;
        if response.code == 0 {
            Ok(())
        } else {
            Err(Error::Api {
                code: response.code,
                message: response.msg,
            })
        }
    }

    pub async fn disable_two_factor(&self) -> Result<(), Error> {
        let response: ApiResponse<()> = self.delete("/user/2fa/disable").await?;
        if response.code == 0 {
            Ok(())
        } else {
            Err(Error::Api {
                code: response.code,
                message: response.msg,
            })
        }
    }

    pub async fn get_user_avatar(&self, user_id: &str) -> Result<String, Error> {
        let endpoint = format!("/user/avatar/{}", user_id);
        let response: ApiResponse<String> = self.get(&endpoint).await?;
        Ok(response.data.unwrap_or_default())
    }

    pub async fn get_user_info(&self, user_id: &str) -> Result<User, Error> {
        let endpoint = format!("/user/info/{}", user_id);
        let response: ApiResponse<User> = self.get(&endpoint).await?;
        Ok(response.data.unwrap())
    }

    pub async fn get_user_shares(
        &self,
        user_id: &str,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<Vec<ShareLink>, Error> {
        let mut endpoint = format!("/user/shares/{}", user_id);
        let mut query_params = Vec::new();
        if let Some(p) = page {
            query_params.push(format!("page={}", p));
        }
        if let Some(ps) = page_size {
            query_params.push(format!("page_size={}", ps));
        }

        if !query_params.is_empty() {
            endpoint = format!("{}?{}", endpoint, query_params.join("&"));
        }

        let response: ApiResponse<Vec<ShareLink>> = self.get(&endpoint).await?;
        Ok(response.data.unwrap())
    }

    pub async fn update_profile(&self, request: &UpdateProfileRequest<'_>) -> Result<User, Error> {
        let response: ApiResponse<User> = self.put("/user/profile", request).await?;
        Ok(response.data.unwrap())
    }

    pub async fn change_password(
        &self,
        old_password: &str,
        new_password: &str,
    ) -> Result<(), Error> {
        let request = ChangePasswordRequest {
            old_password,
            new_password,
        };
        let _: ApiResponse<()> = self.post("/user/password", &request).await?;
        Ok(())
    }

    pub async fn get_quota(&self) -> Result<Quota, Error> {
        let response: ApiResponse<Quota> = self.get("/user/quota").await?;
        Ok(response.data.unwrap())
    }

    pub async fn get_settings(&self) -> Result<UserSettings, Error> {
        let response: ApiResponse<UserSettings> = self.get("/user/setting").await?;
        Ok(response.data.unwrap())
    }

    pub async fn update_settings(&self, settings: &UserSettings) -> Result<UserSettings, Error> {
        let response: ApiResponse<UserSettings> = self.put("/user/setting", settings).await?;
        Ok(response.data.unwrap())
    }

    pub async fn get_storage_policies(&self) -> Result<Vec<StoragePolicy>, Error> {
        let response: ApiResponse<Vec<StoragePolicy>> = self.get("/user/setting/policies").await?;

        // Check for API error response
        if response.code != 0 {
            return Err(Error::Api {
                code: response.code,
                message: response.msg,
            });
        }

        Ok(response.data.unwrap_or_default())
    }
}
