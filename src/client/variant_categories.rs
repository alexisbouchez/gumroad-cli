use crate::error::Result;
use crate::models::variant_category::VariantCategory;

use super::GumroadClient;

impl GumroadClient {
    pub async fn list_variant_categories(
        &self,
        product_id: &str,
    ) -> Result<Vec<VariantCategory>> {
        let body = self
            .get(&format!("/v2/products/{product_id}/variant_categories"))
            .await?;
        let categories: Vec<VariantCategory> =
            serde_json::from_value(body["variant_categories"].clone())?;
        Ok(categories)
    }

    pub async fn get_variant_category(
        &self,
        product_id: &str,
        id: &str,
    ) -> Result<VariantCategory> {
        let body = self
            .get(&format!(
                "/v2/products/{product_id}/variant_categories/{id}"
            ))
            .await?;
        let category: VariantCategory =
            serde_json::from_value(body["variant_category"].clone())?;
        Ok(category)
    }

    pub async fn create_variant_category(
        &self,
        product_id: &str,
        params: &[(&str, &str)],
    ) -> Result<VariantCategory> {
        let body = self
            .post(
                &format!("/v2/products/{product_id}/variant_categories"),
                params,
            )
            .await?;
        let category: VariantCategory =
            serde_json::from_value(body["variant_category"].clone())?;
        Ok(category)
    }

    pub async fn update_variant_category(
        &self,
        product_id: &str,
        id: &str,
        params: &[(&str, &str)],
    ) -> Result<VariantCategory> {
        let body = self
            .put(
                &format!("/v2/products/{product_id}/variant_categories/{id}"),
                params,
            )
            .await?;
        let category: VariantCategory =
            serde_json::from_value(body["variant_category"].clone())?;
        Ok(category)
    }

    pub async fn delete_variant_category(&self, product_id: &str, id: &str) -> Result<()> {
        self.delete(&format!(
            "/v2/products/{product_id}/variant_categories/{id}"
        ))
        .await?;
        Ok(())
    }
}
