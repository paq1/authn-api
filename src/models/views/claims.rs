use chrono::{Duration, Utc};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub pseudo: String,
    pub id: String
}

impl Claims {
    pub fn new(pseudo: String, id: String) -> Self {
        Self {
            sub: "bla bla".to_string(),
            exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
            pseudo,
            id
        }
    }
}