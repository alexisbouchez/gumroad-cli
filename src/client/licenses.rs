use crate::error::Result;
use crate::models::license::License;

use super::GumroadClient;

impl GumroadClient {
    pub async fn verify_license(
        &self,
        product_id: &str,
        license_key: &str,
        increment_uses_count: bool,
    ) -> Result<License> {
        let inc = increment_uses_count.to_string();
        let body = self
            .post(
                "/v2/licenses/verify",
                &[
                    ("product_id", product_id),
                    ("license_key", license_key),
                    ("increment_uses_count", &inc),
                ],
            )
            .await?;
        let license: License = serde_json::from_value(body.clone())?;
        Ok(license)
    }

    pub async fn enable_license(
        &self,
        product_id: &str,
        license_key: &str,
    ) -> Result<License> {
        let body = self
            .put(
                "/v2/licenses/enable",
                &[
                    ("product_id", product_id),
                    ("license_key", license_key),
                ],
            )
            .await?;
        let license: License = serde_json::from_value(body.clone())?;
        Ok(license)
    }

    pub async fn disable_license(
        &self,
        product_id: &str,
        license_key: &str,
    ) -> Result<License> {
        let body = self
            .put(
                "/v2/licenses/disable",
                &[
                    ("product_id", product_id),
                    ("license_key", license_key),
                ],
            )
            .await?;
        let license: License = serde_json::from_value(body.clone())?;
        Ok(license)
    }

    pub async fn decrement_license_uses(
        &self,
        product_id: &str,
        license_key: &str,
    ) -> Result<License> {
        let body = self
            .put(
                "/v2/licenses/decrement_uses_count",
                &[
                    ("product_id", product_id),
                    ("license_key", license_key),
                ],
            )
            .await?;
        let license: License = serde_json::from_value(body.clone())?;
        Ok(license)
    }

    pub async fn rotate_license(
        &self,
        product_id: &str,
        license_key: &str,
    ) -> Result<License> {
        let body = self
            .put(
                "/v2/licenses/rotate",
                &[
                    ("product_id", product_id),
                    ("license_key", license_key),
                ],
            )
            .await?;
        let license: License = serde_json::from_value(body.clone())?;
        Ok(license)
    }
}
