use crate::error::Result;
use crate::models::variant::Variant;

use super::GumroadClient;

impl GumroadClient {
    pub async fn list_variants(
        &self,
        product_id: &str,
        variant_category_id: &str,
    ) -> Result<Vec<Variant>> {
        let body = self
            .get(&format!(
                "/v2/products/{product_id}/variant_categories/{variant_category_id}/variants"
            ))
            .await?;
        let variants: Vec<Variant> = serde_json::from_value(body["variants"].clone())?;
        Ok(variants)
    }

    pub async fn get_variant(
        &self,
        product_id: &str,
        variant_category_id: &str,
        id: &str,
    ) -> Result<Variant> {
        let body = self
            .get(&format!(
                "/v2/products/{product_id}/variant_categories/{variant_category_id}/variants/{id}"
            ))
            .await?;
        let variant: Variant = serde_json::from_value(body["variant"].clone())?;
        Ok(variant)
    }

    pub async fn create_variant(
        &self,
        product_id: &str,
        variant_category_id: &str,
        params: &[(&str, &str)],
    ) -> Result<Variant> {
        let body = self
            .post(
                &format!(
                    "/v2/products/{product_id}/variant_categories/{variant_category_id}/variants"
                ),
                params,
            )
            .await?;
        let variant: Variant = serde_json::from_value(body["variant"].clone())?;
        Ok(variant)
    }

    pub async fn update_variant(
        &self,
        product_id: &str,
        variant_category_id: &str,
        id: &str,
        params: &[(&str, &str)],
    ) -> Result<Variant> {
        let body = self
            .put(
                &format!(
                    "/v2/products/{product_id}/variant_categories/{variant_category_id}/variants/{id}"
                ),
                params,
            )
            .await?;
        let variant: Variant = serde_json::from_value(body["variant"].clone())?;
        Ok(variant)
    }

    pub async fn delete_variant(
        &self,
        product_id: &str,
        variant_category_id: &str,
        id: &str,
    ) -> Result<()> {
        self.delete(&format!(
            "/v2/products/{product_id}/variant_categories/{variant_category_id}/variants/{id}"
        ))
        .await?;
        Ok(())
    }
}
