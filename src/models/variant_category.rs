use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
pub struct VariantCategory {
    pub id: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Serialize, Tabled)]
pub struct VariantCategoryRow {
    pub id: String,
    pub title: String,
}

impl From<VariantCategory> for VariantCategoryRow {
    fn from(v: VariantCategory) -> Self {
        Self {
            id: v.id.unwrap_or_default(),
            title: v.title.unwrap_or_default(),
        }
    }
}
