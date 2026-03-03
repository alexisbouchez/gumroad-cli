use crate::error::Result;
use crate::models::resource_subscription::ResourceSubscription;

use super::GumroadClient;

impl GumroadClient {
    pub async fn list_resource_subscriptions(&self) -> Result<Vec<ResourceSubscription>> {
        let body = self.get("/v2/resource_subscriptions").await?;
        let subs: Vec<ResourceSubscription> =
            serde_json::from_value(body["resource_subscriptions"].clone())?;
        Ok(subs)
    }

    pub async fn create_resource_subscription(
        &self,
        params: &[(&str, &str)],
    ) -> Result<ResourceSubscription> {
        let body = self
            .post("/v2/resource_subscriptions", params)
            .await?;
        let sub: ResourceSubscription =
            serde_json::from_value(body["resource_subscription"].clone())?;
        Ok(sub)
    }

    pub async fn delete_resource_subscription(&self, id: &str) -> Result<()> {
        self.delete(&format!("/v2/resource_subscriptions/{id}"))
            .await?;
        Ok(())
    }
}
