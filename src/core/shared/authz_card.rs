use rocket::serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthzCard {
    pub description: String,
    pub resource: String,
    pub action: String,
    pub users: Vec<String>
}
