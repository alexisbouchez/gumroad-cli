use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
pub struct Payout {
    pub id: Option<String>,
    pub amount_cents: Option<u64>,
    pub user_id: Option<String>,
    pub payout_period_start_date: Option<String>,
    pub payout_period_end_date: Option<String>,
    pub display_amount: Option<String>,
    pub is_paid: Option<bool>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Tabled)]
pub struct PayoutRow {
    pub id: String,
    pub amount: String,
    pub status: String,
    pub period_start: String,
    pub period_end: String,
}

impl From<Payout> for PayoutRow {
    fn from(p: Payout) -> Self {
        Self {
            id: p.id.unwrap_or_default(),
            amount: p.display_amount.unwrap_or_default(),
            status: p.status.unwrap_or_else(|| {
                p.is_paid
                    .map(|b| if b { "Paid" } else { "Unpaid" }.to_string())
                    .unwrap_or_default()
            }),
            period_start: p.payout_period_start_date.unwrap_or_default(),
            period_end: p.payout_period_end_date.unwrap_or_default(),
        }
    }
}
