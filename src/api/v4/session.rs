//! Session and authentication management for Cloudreve API v4

use crate::Error;
use crate::api::v4::ApiV4Client;
use crate::api::v4::models::*;
use serde::Serialize;

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
        // V4 login returns 203 status code when 2FA is required
        // In this case, data contains the session ID as a string
        let response_text = self.post_raw("/session/token", request).await?;

        log::debug!("Raw login response: {}", response_text);

        // First, parse as ApiResponse<String> to check for 2FA (code 203)
        // When code is 203, data is a string (session ID)
        if let Ok(api_response) = serde_json::from_str::<ApiResponse<String>>(&response_text) {
            if api_response.code == 203 {
                // 2FA required - data contains session ID
                let session_id = api_response.data.unwrap_or_default();
                log::debug!("2FA required, session ID: {}", session_id);
                return Err(crate::Error::TwoFactorRequired(session_id));
            }

            // Check for other error codes
            if api_response.code != 0 {
                return Err(crate::Error::Api {
                    code: api_response.code,
                    message: api_response.msg,
                });
            }
        }

        // If not 2FA, parse as LoginData
        match serde_json::from_str::<ApiResponse<LoginData>>(&response_text) {
            Ok(api_response) => {
                // Check for other error codes
                if api_response.code != 0 {
                    return Err(crate::Error::Api {
                        code: api_response.code,
                        message: api_response.msg,
                    });
                }

                match api_response.data {
                    Some(data) => Ok(data),
                    None => Err(crate::Error::InvalidResponse(
                        "Missing login data in response".to_string(),
                    )),
                }
            }
            Err(e) => {
                log::error!("Failed to parse login response: {}", e);
                log::error!("Response text: {}", response_text);
                Err(crate::Error::InvalidResponse(format!(
                    "Invalid login response format: {}",
                    e
                )))
            }
        }
    }

    /// Post a request and return the raw response text
    async fn post_raw(&self, endpoint: &str, body: &impl Serialize) -> Result<String, Error> {
        let url = self.get_url(endpoint);
        let mut http_request = self.http_client.post(&url).json(body);

        if let Some(token) = &self.token {
            http_request = http_request.header("Authorization", format!("Bearer {}", token));
        }

        let response = http_request.send().await?;
        let status = response.status();

        let raw_text = response.text().await?;

        if !status.is_success() {
            return Err(crate::Error::Api {
                code: status.as_u16() as i32,
                message: raw_text.trim().to_string(),
            });
        }

        Ok(raw_text)
    }

    pub async fn finish_2fa_login(
        &self,
        request: &TwoFactorLoginRequest<'_>,
    ) -> Result<LoginData, Error> {
        let response: ApiResponse<LoginData> = self.post("/session/token/2fa", request).await?;
        match response.data {
            Some(data) => Ok(data),
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
