use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
pub struct Variant {
    pub id: Option<String>,
    pub name: Option<String>,
    pub price_difference: Option<i64>,
    pub max_purchase_count: Option<u64>,
}

#[derive(Debug, Serialize, Tabled)]
pub struct VariantRow {
    pub id: String,
    pub name: String,
    pub price_difference: String,
    pub max_purchase_count: String,
}

impl From<Variant> for VariantRow {
    fn from(v: Variant) -> Self {
        Self {
            id: v.id.unwrap_or_default(),
            name: v.name.unwrap_or_default(),
            price_difference: v
                .price_difference
                .map(|p| p.to_string())
                .unwrap_or_default(),
            max_purchase_count: v
                .max_purchase_count
                .map(|c| c.to_string())
                .unwrap_or_default(),
        }
    }
}
