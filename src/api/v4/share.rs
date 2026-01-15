use crate::api::v4::models::*;
use crate::api::v4::uri::path_to_uri;
use crate::api::v4::ApiV4Client;
use crate::Error;
use serde_json::Value;

/// Share management methods
impl ApiV4Client {
    pub async fn create_share_link(
        &self,
        request: &CreateShareLinkRequest,
    ) -> Result<String, Error> {
        // Convert URI format internally
        let uri = path_to_uri(&request.uri);
        let converted_request = CreateShareLinkRequest {
            uri,
            permissions: request.permissions.clone(),
            is_private: request.is_private,
            share_view: request.share_view,
            expire: request.expire,
            price: request.price,
            password: request.password.clone(),
            show_readme: request.show_readme,
        };

        let response: ApiResponse<String> = self.put("/share", &converted_request).await?;
        response.data.ok_or_else(|| {
            Error::InvalidResponse(format!(
                "API returned error: code={}, msg={}",
                response.code, response.msg
            ))
        })
    }

    pub async fn list_my_share_links_with_params(
        &self,
        page_size: u32,
        order_by: Option<&str>,
        order_direction: Option<&str>,
        next_page_token: Option<&str>,
    ) -> Result<(Vec<ShareLink>, Option<String>), Error> {
        let mut query_params = vec![("page_size", page_size.to_string())];
        if let Some(order_by) = order_by {
            query_params.push(("order_by", order_by.to_string()));
        }
        if let Some(order_direction) = order_direction {
            query_params.push(("order_direction", order_direction.to_string()));
        }
        if let Some(next_page_token) = next_page_token {
            query_params.push(("next_page_token", next_page_token.to_string()));
        }

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");

        let endpoint = format!("/share?{}", query_string);
        let response: ApiResponse<Value> = self.get(&endpoint).await?;

        if let Some(data) = response.data {
            let shares: Vec<ShareLink> =
                serde_json::from_value(data.get("shares").unwrap_or(&Value::Array(vec![])).clone())
                    .unwrap_or_default();

            let next_token = data
                .get("pagination")
                .and_then(|p| p.get("next_page_token"))
                .and_then(|t| t.as_str())
                .map(|s| s.to_string());

            Ok((shares, next_token))
        } else {
            // Return error if API returned an error response
            Err(Error::InvalidResponse(format!(
                "API returned error: code={}, msg={}",
                response.code, response.msg
            )))
        }
    }

    pub async fn list_my_share_links(&self) -> Result<Vec<ShareLink>, Error> {
        let (shares, _) = self
            .list_my_share_links_with_params(50, None, None, None)
            .await?;
        Ok(shares)
    }

    pub async fn edit_share_link(
        &self,
        share_id: &str,
        request: &EditShareLinkRequest,
    ) -> Result<ShareLink, Error> {
        let response: ApiResponse<ShareLink> =
            self.post(&format!("/share/{}", share_id), request).await?;
        response.data.ok_or_else(|| {
            Error::InvalidResponse(format!(
                "API returned error: code={}, msg={}",
                response.code, response.msg
            ))
        })
    }

    pub async fn delete_share_link(&self, share_id: &str) -> Result<(), Error> {
        let _: ApiResponse<Value> = self.delete(&format!("/share/{}", share_id)).await?;
        Ok(())
    }

    pub async fn get_share_link_info(
        &self,
        share_id: &str,
        password: Option<&str>,
        count_views: Option<bool>,
        owner_extended: Option<bool>,
    ) -> Result<ShareLink, Error> {
        let mut query_params = Vec::new();
        if let Some(password) = password {
            query_params.push(format!("password={}", password));
        }
        if let Some(count_views) = count_views {
            query_params.push(format!("count_views={}", count_views));
        }
        if let Some(owner_extended) = owner_extended {
            query_params.push(format!("owner_extended={}", owner_extended));
        }

        let query_string = if !query_params.is_empty() {
            format!("?{}", query_params.join("&"))
        } else {
            String::new()
        };

        let endpoint = format!("/share/info/{}{}", share_id, query_string);
        let response: ApiResponse<Value> = self.get(&endpoint).await?;

        if let Some(data) = response.data {
            let share: ShareLink = serde_json::from_value(data).unwrap();
            Ok(share)
        } else {
            Err(Error::InvalidResponse("No data in response".to_string()))
        }
    }

    pub async fn report_abuse(&self, share_id: &str, reason: &str) -> Result<(), Error> {
        let request = AbuseReportRequest { reason };
        let _: ApiResponse<()> = self
            .post(&format!("/share/{}/report", share_id), &request)
            .await?;
        Ok(())
    }
}
