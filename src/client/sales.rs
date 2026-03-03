use crate::error::Result;
use crate::models::sale::Sale;

use super::GumroadClient;

pub struct SalesPage {
    pub sales: Vec<Sale>,
    pub next_page_key: Option<String>,
}

impl GumroadClient {
    pub async fn list_sales(&self, params: &[(&str, &str)]) -> Result<SalesPage> {
        let body = self.get_with_params("/v2/sales", params).await?;
        let sales: Vec<Sale> = serde_json::from_value(body["sales"].clone())?;
        let next_page_key = body["next_page_key"]
            .as_str()
            .map(|s| s.to_string());
        Ok(SalesPage {
            sales,
            next_page_key,
        })
    }

    pub async fn get_sale(&self, id: &str) -> Result<Sale> {
        let body = self.get(&format!("/v2/sales/{id}")).await?;
        let sale: Sale = serde_json::from_value(body["sale"].clone())?;
        Ok(sale)
    }

    pub async fn mark_sale_as_shipped(
        &self,
        id: &str,
        tracking_url: Option<&str>,
    ) -> Result<Sale> {
        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(url) = tracking_url {
            params.push(("tracking_url", url));
        }
        let body = self
            .put(&format!("/v2/sales/{id}/mark_as_shipped"), &params)
            .await?;
        let sale: Sale = serde_json::from_value(body["sale"].clone())?;
        Ok(sale)
    }

    pub async fn refund_sale(&self, id: &str, amount_cents: Option<&str>) -> Result<Sale> {
        let mut params: Vec<(&str, &str)> = vec![];
        if let Some(amount) = amount_cents {
            params.push(("amount_cents", amount));
        }
        let body = self
            .put(&format!("/v2/sales/{id}/refund"), &params)
            .await?;
        let sale: Sale = serde_json::from_value(body["sale"].clone())?;
        Ok(sale)
    }

    pub async fn resend_receipt(&self, id: &str) -> Result<()> {
        self.post(&format!("/v2/sales/{id}/resend_receipt"), &[])
            .await?;
        Ok(())
    }
}
