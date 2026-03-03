use crate::error::Result;
use crate::models::custom_field::CustomField;

use super::GumroadClient;

impl GumroadClient {
    pub async fn list_custom_fields(&self, product_id: &str) -> Result<Vec<CustomField>> {
        let body = self
            .get(&format!("/v2/products/{product_id}/custom_fields"))
            .await?;
        let fields: Vec<CustomField> =
            serde_json::from_value(body["custom_fields"].clone())?;
        Ok(fields)
    }

    pub async fn create_custom_field(
        &self,
        product_id: &str,
        params: &[(&str, &str)],
    ) -> Result<CustomField> {
        let body = self
            .post(
                &format!("/v2/products/{product_id}/custom_fields"),
                params,
            )
            .await?;
        let field: CustomField = serde_json::from_value(body["custom_field"].clone())?;
        Ok(field)
    }

    pub async fn update_custom_field(
        &self,
        product_id: &str,
        name: &str,
        params: &[(&str, &str)],
    ) -> Result<CustomField> {
        let body = self
            .put(
                &format!("/v2/products/{product_id}/custom_fields/{name}"),
                params,
            )
            .await?;
        let field: CustomField = serde_json::from_value(body["custom_field"].clone())?;
        Ok(field)
    }

    pub async fn delete_custom_field(&self, product_id: &str, name: &str) -> Result<()> {
        self.delete(&format!(
            "/v2/products/{product_id}/custom_fields/{name}"
        ))
        .await?;
        Ok(())
    }
}
