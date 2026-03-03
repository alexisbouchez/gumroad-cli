use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
pub struct Sku {
    pub id: Option<String>,
    pub name: Option<String>,
    pub price_difference: Option<i64>,
    pub max_purchase_count: Option<u64>,
    pub is_default: Option<bool>,
}

#[derive(Debug, Serialize, Tabled)]
pub struct SkuRow {
    pub id: String,
    pub name: String,
    pub price_difference: String,
    pub max_purchase_count: String,
}

impl From<Sku> for SkuRow {
    fn from(s: Sku) -> Self {
        Self {
            id: s.id.unwrap_or_default(),
            name: s.name.unwrap_or_default(),
            price_difference: s
                .price_difference
                .map(|p| p.to_string())
                .unwrap_or_default(),
            max_purchase_count: s
                .max_purchase_count
                .map(|c| c.to_string())
                .unwrap_or_default(),
        }
    }
}
