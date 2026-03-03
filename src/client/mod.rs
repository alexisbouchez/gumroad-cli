pub mod custom_fields;
pub mod licenses;
pub mod offer_codes;
pub mod payouts;
pub mod products;
pub mod resource_subscriptions;
pub mod sales;
pub mod skus;
pub mod subscribers;
pub mod user;
pub mod variant_categories;
pub mod variants;

use reqwest::Client;

use crate::error::{GumroadError, Result};

const BASE_URL: &str = "https://api.gumroad.com";

pub struct GumroadClient {
    http: Client,
    base_url: String,
    access_token: String,
}

impl GumroadClient {
    pub fn new(access_token: String) -> Self {
        Self {
            http: Client::new(),
            base_url: BASE_URL.to_string(),
            access_token,
        }
    }

    #[cfg(test)]
    pub fn with_base_url(access_token: String, base_url: String) -> Self {
        Self {
            http: Client::new(),
            base_url,
            access_token,
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{path}", self.base_url)
    }

    pub async fn get(&self, path: &str) -> Result<serde_json::Value> {
        let resp = self
            .http
            .get(self.url(path))
            .query(&[("access_token", &self.access_token)])
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    pub async fn get_with_params(
        &self,
        path: &str,
        params: &[(&str, &str)],
    ) -> Result<serde_json::Value> {
        let mut all_params: Vec<(&str, &str)> = vec![("access_token", &self.access_token)];
        all_params.extend_from_slice(params);
        let resp = self
            .http
            .get(self.url(path))
            .query(&all_params)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    pub async fn post(
        &self,
        path: &str,
        form: &[(&str, &str)],
    ) -> Result<serde_json::Value> {
        let mut all_form: Vec<(&str, &str)> = vec![("access_token", &self.access_token)];
        all_form.extend_from_slice(form);
        let resp = self.http.post(self.url(path)).form(&all_form).send().await?;
        Self::handle_response(resp).await
    }

    pub async fn put(
        &self,
        path: &str,
        form: &[(&str, &str)],
    ) -> Result<serde_json::Value> {
        let mut all_form: Vec<(&str, &str)> = vec![("access_token", &self.access_token)];
        all_form.extend_from_slice(form);
        let resp = self.http.put(self.url(path)).form(&all_form).send().await?;
        Self::handle_response(resp).await
    }

    pub async fn delete(&self, path: &str) -> Result<serde_json::Value> {
        let resp = self
            .http
            .delete(self.url(path))
            .query(&[("access_token", &self.access_token)])
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    async fn handle_response(resp: reqwest::Response) -> Result<serde_json::Value> {
        let status = resp.status();
        let body: serde_json::Value = resp.json().await?;

        if !status.is_success() {
            let message = body["message"]
                .as_str()
                .unwrap_or("Unknown API error")
                .to_string();
            return Err(GumroadError::Api(message));
        }

        if body.get("success") == Some(&serde_json::Value::Bool(false)) {
            let message = body["message"]
                .as_str()
                .unwrap_or("Unknown API error")
                .to_string();
            return Err(GumroadError::Api(message));
        }

        Ok(body)
    }
}
