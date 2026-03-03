use crate::error::Result;
use crate::models::user::User;

use super::GumroadClient;

impl GumroadClient {
    pub async fn get_user(&self) -> Result<User> {
        let body = self.get("/v2/user").await?;
        let user: User = serde_json::from_value(body["user"].clone())?;
        Ok(user)
    }
}
