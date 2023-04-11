use mongodb::bson::{Bson, Document};
use mongodb::bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};

use crate::core::entities::user::User;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserDbo {
    pub _id: ObjectId,
    pub id: String,
    pub pseudo: String,
    pub mdp: String
}

impl From<UserDbo> for User {
    fn from(value: UserDbo) -> Self {
        User {
            id: value.id,
            pseudo: value.pseudo,
            mdp: value.mdp
        }
    }
}

impl From<Document> for UserDbo {
    fn from(value: Document) -> Self {
        mongodb::bson::from_bson(Bson::Document(value))
            .unwrap()
    }
}