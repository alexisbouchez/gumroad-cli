use crate::error::Result;
use crate::models::sku::Sku;

use super::GumroadClient;

impl GumroadClient {
    pub async fn list_skus(&self, product_id: &str) -> Result<Vec<Sku>> {
        let body = self
            .get(&format!("/v2/products/{product_id}/skus"))
            .await?;
        let skus: Vec<Sku> = serde_json::from_value(body["skus"].clone())?;
        Ok(skus)
    }
}
