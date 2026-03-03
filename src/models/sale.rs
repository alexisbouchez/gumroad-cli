use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
pub struct Sale {
    pub id: Option<String>,
    pub email: Option<String>,
    pub seller_id: Option<String>,
    pub product_id: Option<String>,
    pub product_name: Option<String>,
    pub price: Option<u64>,
    pub formatted_display_price: Option<String>,
    pub currency: Option<String>,
    pub quantity: Option<u64>,
    pub order_number: Option<u64>,
    pub sale_timestamp: Option<String>,
    pub refunded: Option<bool>,
    pub disputed: Option<bool>,
    pub shipped: Option<bool>,
    pub license_key: Option<String>,
}

#[derive(Debug, Serialize, Tabled)]
pub struct SaleRow {
    pub id: String,
    pub product_name: String,
    pub email: String,
    pub price: String,
    pub refunded: String,
    pub timestamp: String,
}

impl From<Sale> for SaleRow {
    fn from(s: Sale) -> Self {
        Self {
            id: s.id.unwrap_or_default(),
            product_name: s.product_name.unwrap_or_default(),
            email: s.email.unwrap_or_default(),
            price: s.formatted_display_price.unwrap_or_default(),
            refunded: s
                .refunded
                .map(|b| if b { "Yes" } else { "No" }.to_string())
                .unwrap_or_default(),
            timestamp: s.sale_timestamp.unwrap_or_default(),
        }
    }
}
