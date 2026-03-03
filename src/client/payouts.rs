use crate::error::Result;
use crate::models::payout::Payout;

use super::GumroadClient;

impl GumroadClient {
    pub async fn list_payouts(&self, params: &[(&str, &str)]) -> Result<Vec<Payout>> {
        let body = self.get_with_params("/v2/payouts", params).await?;
        let payouts: Vec<Payout> = serde_json::from_value(body["payouts"].clone())?;
        Ok(payouts)
    }

    pub async fn get_payout(&self, id: &str) -> Result<Payout> {
        let body = self.get(&format!("/v2/payouts/{id}")).await?;
        let payout: Payout = serde_json::from_value(body["payout"].clone())?;
        Ok(payout)
    }

    pub async fn get_upcoming_payout(&self) -> Result<Payout> {
        let body = self.get("/v2/payouts/upcoming").await?;
        let payout: Payout = serde_json::from_value(body["payout"].clone())?;
        Ok(payout)
    }
}
