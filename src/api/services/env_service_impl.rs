use crate::core::services::env_service::EnvService;

pub struct EnvServiceImpl;

impl EnvService for EnvServiceImpl {
    fn get_salt(&self) -> String {
        std::env::var("SECRET_SALT")
            .expect("Pas de SECRET_SALT dans les variables d'environnement.")
    }
}