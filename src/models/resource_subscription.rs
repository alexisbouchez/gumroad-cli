use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
pub struct ResourceSubscription {
    pub id: Option<String>,
    pub resource_name: Option<String>,
    pub post_url: Option<String>,
}

#[derive(Debug, Serialize, Tabled)]
pub struct ResourceSubscriptionRow {
    pub id: String,
    pub resource_name: String,
    pub post_url: String,
}

impl From<ResourceSubscription> for ResourceSubscriptionRow {
    fn from(r: ResourceSubscription) -> Self {
        Self {
            id: r.id.unwrap_or_default(),
            resource_name: r.resource_name.unwrap_or_default(),
            post_url: r.post_url.unwrap_or_default(),
        }
    }
}
