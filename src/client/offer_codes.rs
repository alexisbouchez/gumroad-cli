use crate::error::Result;
use crate::models::offer_code::OfferCode;

use super::GumroadClient;

impl GumroadClient {
    pub async fn list_offer_codes(&self, product_id: &str) -> Result<Vec<OfferCode>> {
        let body = self
            .get(&format!("/v2/products/{product_id}/offer_codes"))
            .await?;
        let codes: Vec<OfferCode> = serde_json::from_value(body["offer_codes"].clone())?;
        Ok(codes)
    }

    pub async fn get_offer_code(&self, product_id: &str, id: &str) -> Result<OfferCode> {
        let body = self
            .get(&format!("/v2/products/{product_id}/offer_codes/{id}"))
            .await?;
        let code: OfferCode = serde_json::from_value(body["offer_code"].clone())?;
        Ok(code)
    }

    pub async fn create_offer_code(
        &self,
        product_id: &str,
        params: &[(&str, &str)],
    ) -> Result<OfferCode> {
        let body = self
            .post(
                &format!("/v2/products/{product_id}/offer_codes"),
                params,
            )
            .await?;
        let code: OfferCode = serde_json::from_value(body["offer_code"].clone())?;
        Ok(code)
    }

    pub async fn update_offer_code(
        &self,
        product_id: &str,
        id: &str,
        params: &[(&str, &str)],
    ) -> Result<OfferCode> {
        let body = self
            .put(
                &format!("/v2/products/{product_id}/offer_codes/{id}"),
                params,
            )
            .await?;
        let code: OfferCode = serde_json::from_value(body["offer_code"].clone())?;
        Ok(code)
    }

    pub async fn delete_offer_code(&self, product_id: &str, id: &str) -> Result<()> {
        self.delete(&format!(
            "/v2/products/{product_id}/offer_codes/{id}"
        ))
        .await?;
        Ok(())
    }
}
