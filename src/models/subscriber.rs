use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
pub struct Subscriber {
    pub id: Option<String>,
    pub product_id: Option<String>,
    pub product_name: Option<String>,
    pub user_id: Option<String>,
    pub user_email: Option<String>,
    pub charge_occurrence_count: Option<u64>,
    pub status: Option<String>,
    pub created_at: Option<String>,
    pub ended_at: Option<String>,
    pub failed_at: Option<String>,
    pub free_trial_ends_at: Option<String>,
    pub license_key: Option<String>,
}

#[derive(Debug, Serialize, Tabled)]
pub struct SubscriberRow {
    pub id: String,
    pub email: String,
    pub status: String,
    pub product_name: String,
    pub created_at: String,
}

impl From<Subscriber> for SubscriberRow {
    fn from(s: Subscriber) -> Self {
        Self {
            id: s.id.unwrap_or_default(),
            email: s.user_email.unwrap_or_default(),
            status: s.status.unwrap_or_default(),
            product_name: s.product_name.unwrap_or_default(),
            created_at: s.created_at.unwrap_or_default(),
        }
    }
}
