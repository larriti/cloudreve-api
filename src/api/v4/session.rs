//! Session and authentication management for Cloudreve API v4

use crate::Error;
use crate::api::v4::ApiV4Client;
use crate::api::v4::models::*;

impl ApiV4Client {
    pub async fn prepare_login(&self, email: &str) -> Result<LoginPreparation, Error> {
        let endpoint = format!("/session/prepare?email={}", email);
        let response: crate::ApiResponse<LoginPreparation> = self.get(&endpoint).await?;
        match response.data {
            Some(preparation) => Ok(preparation),
            None => Err(crate::Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }

    pub async fn prepare_openid_signin(
        &self,
        request: &OpenIdPrepareRequest<'_>,
    ) -> Result<String, Error> {
        let response: crate::ApiResponse<String> = self.put("/session/openid", request).await?;
        match response.data {
            Some(url) => Ok(url),
            None => Err(crate::Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }

    pub async fn finish_openid_signin(
        &self,
        request: &OpenIdFinishRequest<'_>,
    ) -> Result<LoginResponse, Error> {
        let response: crate::ApiResponse<LoginResponse> =
            self.post("/session/openid", request).await?;
        match response.data {
            Some(login_response) => Ok(login_response),
            None => Err(crate::Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }

    pub async fn unlink_openid(&self, provider_id: i32) -> Result<(), Error> {
        let endpoint = format!("/session/openid/{}", provider_id);
        let response: crate::ApiResponse<()> = self.delete(&endpoint).await?;
        if response.code == 0 {
            Ok(())
        } else {
            Err(crate::Error::Api {
                code: response.code,
                message: response.msg,
            })
        }
    }

    pub async fn prepare_passkey_signin(&self) -> Result<PasskeySignInPreparation, Error> {
        let response: crate::ApiResponse<PasskeySignInPreparation> =
            self.put("/session/authn", &()).await?;
        match response.data {
            Some(preparation) => Ok(preparation),
            None => Err(crate::Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }

    pub async fn finish_passkey_signin(
        &self,
        request: &PasskeySignInRequest<'_>,
    ) -> Result<LoginResponse, Error> {
        let response: crate::ApiResponse<LoginResponse> =
            self.post("/session/authn", request).await?;
        match response.data {
            Some(login_response) => Ok(login_response),
            None => Err(crate::Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }

    pub async fn login(&self, request: &LoginRequest<'_>) -> Result<LoginData, Error> {
        let response: ApiResponse<LoginData> = self.post("/session/token", request).await?;
        match response.data {
            Some(data) => Ok(data),
            None => Err(crate::Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }

    pub async fn finish_2fa_login(
        &self,
        request: &TwoFactorLoginRequest<'_>,
    ) -> Result<Token, Error> {
        let response: ApiResponse<Token> = self.post("/session/token/2fa", request).await?;
        match response.data {
            Some(token) => Ok(token),
            None => Err(crate::Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }

    pub async fn refresh_token(&self, request: &RefreshTokenRequest<'_>) -> Result<Token, Error> {
        let response: crate::ApiResponse<Token> =
            self.post("/session/token/refresh", request).await?;
        match response.data {
            Some(token) => Ok(token),
            None => Err(crate::Error::Api {
                code: response.code,
                message: response.msg,
            }),
        }
    }

    pub async fn logout(&self) -> Result<(), Error> {
        let _: crate::ApiResponse<()> = self.delete("/session/token").await?;
        Ok(())
    }
}
