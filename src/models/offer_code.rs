use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
pub struct OfferCode {
    pub id: Option<String>,
    pub name: Option<String>,
    pub amount_off: Option<u64>,
    pub offer_type: Option<String>,
    pub max_purchase_count: Option<u64>,
    pub times_used: Option<u64>,
    pub universal: Option<bool>,
}

#[derive(Debug, Serialize, Tabled)]
pub struct OfferCodeRow {
    pub id: String,
    pub name: String,
    pub amount_off: String,
    pub offer_type: String,
    pub max_purchase_count: String,
    pub times_used: String,
}

impl From<OfferCode> for OfferCodeRow {
    fn from(o: OfferCode) -> Self {
        Self {
            id: o.id.unwrap_or_default(),
            name: o.name.unwrap_or_default(),
            amount_off: o.amount_off.map(|a| a.to_string()).unwrap_or_default(),
            offer_type: o.offer_type.unwrap_or_default(),
            max_purchase_count: o
                .max_purchase_count
                .map(|c| c.to_string())
                .unwrap_or_default(),
            times_used: o
                .times_used
                .map(|t| t.to_string())
                .unwrap_or_default(),
        }
    }
}
