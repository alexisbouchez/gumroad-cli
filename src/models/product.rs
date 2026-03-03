use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
pub struct Product {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<u64>,
    pub currency: Option<String>,
    pub short_url: Option<String>,
    pub formatted_price: Option<String>,
    pub published: Option<bool>,
    pub sales_count: Option<u64>,
    pub sales_usd_cents: Option<u64>,
    pub url: Option<String>,
    pub custom_permalink: Option<String>,
}

#[derive(Debug, Serialize, Tabled)]
pub struct ProductRow {
    pub id: String,
    pub name: String,
    pub price: String,
    pub published: String,
    pub sales_count: String,
    pub url: String,
}

impl From<Product> for ProductRow {
    fn from(p: Product) -> Self {
        Self {
            id: p.id.unwrap_or_default(),
            name: p.name.unwrap_or_default(),
            price: p.formatted_price.unwrap_or_default(),
            published: p
                .published
                .map(|b| if b { "Yes" } else { "No" }.to_string())
                .unwrap_or_default(),
            sales_count: p
                .sales_count
                .map(|c| c.to_string())
                .unwrap_or_default(),
            url: p.short_url.unwrap_or_default(),
        }
    }
}
