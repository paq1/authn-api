pub trait EnvService: Send + Sync {
    fn get_salt(&self) -> String;
    fn get_secret_jwt(&self) -> String;
    fn get_authz_url(&self) -> String;
}