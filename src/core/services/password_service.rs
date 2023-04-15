pub trait PasswordService: Send + Sync {
    fn create_hash_password(&self, password: String) -> String;
    fn verifie(&self, password: String, hash: String) -> bool;
}