use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateAccountCommand {
    pub pseudo: String,
    pub mdp: String,
    // todo champs info personne
}
