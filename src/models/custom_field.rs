use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomField {
    pub name: Option<String>,
    pub required: Option<bool>,
}

#[derive(Debug, Serialize, Tabled)]
pub struct CustomFieldRow {
    pub name: String,
    pub required: String,
}

impl From<CustomField> for CustomFieldRow {
    fn from(f: CustomField) -> Self {
        Self {
            name: f.name.unwrap_or_default(),
            required: f
                .required
                .map(|b| if b { "Yes" } else { "No" }.to_string())
                .unwrap_or_default(),
        }
    }
}
