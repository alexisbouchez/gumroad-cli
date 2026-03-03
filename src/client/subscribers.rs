use crate::error::Result;
use crate::models::subscriber::Subscriber;

use super::GumroadClient;

impl GumroadClient {
    pub async fn list_subscribers(&self, product_id: &str) -> Result<Vec<Subscriber>> {
        let body = self
            .get(&format!("/v2/products/{product_id}/subscribers"))
            .await?;
        let subscribers: Vec<Subscriber> =
            serde_json::from_value(body["subscribers"].clone())?;
        Ok(subscribers)
    }

    pub async fn get_subscriber(&self, id: &str) -> Result<Subscriber> {
        let body = self.get(&format!("/v2/subscribers/{id}")).await?;
        let subscriber: Subscriber = serde_json::from_value(body["subscriber"].clone())?;
        Ok(subscriber)
    }
}
