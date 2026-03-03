use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub bio: Option<String>,
    pub name: Option<String>,
    pub twitter_handle: Option<String>,
    pub user_id: Option<String>,
    pub email: Option<String>,
    pub url: Option<String>,
    pub profile_url: Option<String>,
}

#[derive(Debug, Serialize, Tabled)]
pub struct UserRow {
    pub name: String,
    pub email: String,
    pub user_id: String,
    pub profile_url: String,
}

impl From<User> for UserRow {
    fn from(u: User) -> Self {
        Self {
            name: u.name.unwrap_or_default(),
            email: u.email.unwrap_or_default(),
            user_id: u.user_id.unwrap_or_default(),
            profile_url: u.profile_url.unwrap_or_default(),
        }
    }
}
