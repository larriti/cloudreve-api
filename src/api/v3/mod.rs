//! API v3 implementation

use crate::Error;
use log::debug;
use serde::Serialize;

pub mod aria2;
pub mod directory;
pub mod file;
pub mod models;
pub mod object;
pub mod session;
pub mod share;
pub mod site;
pub mod user;

/// API v3 client structure
#[derive(Debug, Clone)]
pub struct ApiV3Client {
    pub base_url: String,
    pub http_client: reqwest::Client,
    pub session_cookie: Option<String>,
}

impl ApiV3Client {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            http_client: reqwest::Client::new(),
            session_cookie: None,
        }
    }

    pub fn set_session_cookie(&mut self, cookie: String) {
        self.session_cookie = Some(cookie);
    }

    pub fn get_session_cookie(&self) -> Option<&str> {
        self.session_cookie.as_deref()
    }

    pub fn get_url(&self, endpoint: &str) -> String {
        format!(
            "{}/api/v3/{}",
            self.base_url.trim_end_matches('/'),
            endpoint.trim_start_matches('/')
        )
    }

    pub async fn get<T>(&self, endpoint: &str) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        let url = self.get_url(endpoint);
        let mut request = self.http_client.get(&url);

        if let Some(cookie) = &self.session_cookie {
            request = request.header("Cookie", format!("cloudreve-session={}", cookie));
        }

        debug!("GET URL: {}", url);

        let response = request.send().await?;
        let status = response.status();
        let json: T = response.json().await?;
        debug!("Response status: {}, JSON: {:?}", status, json);
        Ok(json)
    }

    pub async fn post<T>(&self, endpoint: &str, body: &impl Serialize) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        let url = self.get_url(endpoint);
        let mut request = self.http_client.post(&url).json(body);

        if let Some(cookie) = &self.session_cookie {
            request = request.header("Cookie", format!("cloudreve-session={}", cookie));
        }

        debug!("POST URL: {}", url);

        let response = request.send().await?;
        let status = response.status();
        let json: T = response.json().await?;
        debug!("Response status: {}, JSON: {:?}", status, json);
        Ok(json)
    }

    pub async fn put<T>(&self, endpoint: &str, body: &impl Serialize) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        let url = self.get_url(endpoint);
        let mut request = self.http_client.put(&url).json(body);

        if let Some(cookie) = &self.session_cookie {
            request = request.header("Cookie", format!("cloudreve-session={}", cookie));
        }

        debug!("PUT URL: {}", url);

        let response = request.send().await?;
        let status = response.status();
        let json: T = response.json().await?;
        debug!("Response status: {}, JSON: {:?}", status, json);
        Ok(json)
    }

    pub async fn patch<T>(&self, endpoint: &str, body: &impl Serialize) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        let url = self.get_url(endpoint);
        let mut request = self.http_client.patch(&url).json(body);

        if let Some(cookie) = &self.session_cookie {
            request = request.header("Cookie", format!("cloudreve-session={}", cookie));
        }

        debug!("PATCH URL: {}", url);

        let response = request.send().await?;
        let status = response.status();
        let json: T = response.json().await?;
        debug!("Response status: {}, JSON: {:?}", status, json);
        Ok(json)
    }

    pub async fn delete<T>(&self, endpoint: &str) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        let url = self.get_url(endpoint);
        let mut request = self.http_client.delete(&url);

        if let Some(cookie) = &self.session_cookie {
            request = request.header("Cookie", format!("cloudreve-session={}", cookie));
        }

        debug!("DELETE URL: {}", url);

        let response = request.send().await?;
        let status = response.status();
        let json: T = response.json().await?;
        debug!("Response status: {}, JSON: {:?}", status, json);
        Ok(json)
    }

    pub async fn delete_with_body<T>(
        &self,
        endpoint: &str,
        body: &impl Serialize,
    ) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        let url = self.get_url(endpoint);
        let mut request = self.http_client.delete(&url).json(body);

        if let Some(cookie) = &self.session_cookie {
            request = request.header("Cookie", format!("cloudreve-session={}", cookie));
        }

        debug!("DELETE WITH BODY URL: {}", url);

        let response = request.send().await?;
        let status = response.status();
        let json: T = response.json().await?;
        debug!("Response status: {}, JSON: {:?}", status, json);
        Ok(json)
    }
}
