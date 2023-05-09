use crate::core::services::env_service::EnvService;

pub struct EnvServiceImpl;

impl EnvService for EnvServiceImpl {
    fn get_salt(&self) -> String {
        std::env::var("SECRET_SALT")
            .expect("Pas de SECRET_SALT dans les variables d'environnement.")
    }

    fn get_secret_jwt(&self) -> String {
        std::env::var("SECRET_ENCODING_KEY")
            .expect("Pas de SECRET_SALT dans les variables d'environnement.")
    }

    fn get_authz_url(&self) -> String {
        std::env::var("AUTHZ_URL")
            .expect("Pas de AUTHZ_URL dans les variables d'environnement.")
    }
}