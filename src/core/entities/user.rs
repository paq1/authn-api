use mongodb::bson::{doc, Document};

#[derive(Clone)]
pub struct User {
    pub id: String,
    pub pseudo: String,
    pub mdp: String
}

impl From<User> for Document {
    fn from(value: User) -> Self {
        doc! {
            "id": value.id,
            "pseudo": value.pseudo,
            "mdp": value.mdp
        }
    }
}