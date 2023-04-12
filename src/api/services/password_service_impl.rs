use bcrypt::{DEFAULT_COST, hash_with_salt};
use crate::core::services::password_service::PasswordService;

pub struct PasswordServiceImpl;

impl PasswordServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl PasswordService for PasswordServiceImpl {
    fn create_hash_password(&self, password: String) -> String {

        let salt: [u8;16] = "azertyuiopqsdfgh"
            .as_bytes()
            .to_vec()[..]
            .try_into()
            .unwrap();

        // let exemple: [u8; 16] = [38, 113, 212, 141, 108, 213, 195, 166, 201, 38, 20, 13, 47, 40, 104, 18];
        // let ex_str = String::from_utf8(exemple.to_vec()).unwrap();

        // println!("sel d'exemple : {}", ex_str);
        println!("mon_encrypt : {:?}", salt.to_vec());
        println!("mon sel decrypt : {}", String::from_utf8(salt.to_vec()).unwrap());


        let hashed = hash_with_salt(
            password,
            DEFAULT_COST,
            salt
            // [38, 113, 212, 141, 108, 213, 195, 166, 201, 38, 20, 13, 47, 40, 104, 18]
        );
        hashed
            .unwrap()
            .to_string()
    }

    fn verifie(&self, password: String, hash: String) -> bool {
        self.create_hash_password(password) == hash
    }
}