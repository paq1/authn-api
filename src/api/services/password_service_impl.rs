use bcrypt::{DEFAULT_COST, hash_with_salt};
use crate::core::services::password_service::PasswordService;

pub struct PasswordServiceImpl;

impl PasswordServiceImpl {
    pub fn new() -> Self {
        Self {}
    }

    pub fn salt_u8_16() -> [u8;16] {
        Self::salt_string_reformatted(16)
            .as_bytes()
            .to_vec()[..16]
            .try_into()
            .unwrap()
    }

    pub fn salt_string_reformatted(length: u32) -> String {
        let salt = std::env::var("SECRET_SALT")
            .expect("Pas de sel dans les venv.");

        Self::pad_with_zero(salt, length)
    }

    pub fn pad_with_zero(chaine: String, length: u32) -> String {
        if chaine.len() >= length as usize {
            chaine
        } else {
            let taille_manquante = length - chaine.len() as u32;
            let fin_de_chaine = (0..taille_manquante)
                .map(|_| "0".to_string())
                .collect::<Vec<String>>()
                .join("");
            chaine.chars()
                .chain(fin_de_chaine.chars())
                .collect::<String>()
        }
    }
}

impl PasswordService for PasswordServiceImpl {
    fn create_hash_password(&self, password: String) -> String {

        let salt: [u8;16] = Self::salt_u8_16();

        let hashed = hash_with_salt(
            password,
            DEFAULT_COST,
            salt
        );
        hashed
            .unwrap()
            .to_string()
    }

    fn verifie(&self, password: String, hash: String) -> bool {
        self.create_hash_password(password) == hash
    }
}