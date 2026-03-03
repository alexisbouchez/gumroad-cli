use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
pub struct License {
    pub uses: Option<u64>,
    pub success: Option<bool>,
    pub purchase: Option<LicensePurchase>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LicensePurchase {
    pub id: Option<String>,
    pub product_id: Option<String>,
    pub product_name: Option<String>,
    pub email: Option<String>,
    pub seller_id: Option<String>,
    pub license_key: Option<String>,
    pub uses: Option<u64>,
    pub refunded: Option<bool>,
    pub disputed: Option<bool>,
    pub chargebacked: Option<bool>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Tabled)]
pub struct LicenseRow {
    pub license_key: String,
    pub email: String,
    pub product_name: String,
    pub uses: String,
    pub created_at: String,
}

impl From<License> for LicenseRow {
    fn from(l: License) -> Self {
        let p = l.purchase.unwrap_or(LicensePurchase {
            id: None,
            product_id: None,
            product_name: None,
            email: None,
            seller_id: None,
            license_key: None,
            uses: None,
            refunded: None,
            disputed: None,
            chargebacked: None,
            created_at: None,
        });
        Self {
            license_key: p.license_key.unwrap_or_default(),
            email: p.email.unwrap_or_default(),
            product_name: p.product_name.unwrap_or_default(),
            uses: l.uses.or(p.uses).map(|u| u.to_string()).unwrap_or_default(),
            created_at: p.created_at.unwrap_or_default(),
        }
    }
}
