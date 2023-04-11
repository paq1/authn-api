use bcrypt::{DEFAULT_COST, hash, hash_with_salt, verify};
use crate::core::services::password_service::PasswordService;

pub struct PasswordServiceImpl;

impl PasswordServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl PasswordService for PasswordServiceImpl {
    fn create_hash_password(&self, password: String) -> String {
        let hashed = hash_with_salt(password, DEFAULT_COST, [38, 113, 212, 141, 108, 213, 195, 166, 201, 38, 20, 13, 47, 40, 104, 18]);
        hashed
            .unwrap()
            .to_string()
    }

    fn verifie(&self, password: String, hash: String) -> bool {
        self.create_hash_password(password) == hash
    }
}