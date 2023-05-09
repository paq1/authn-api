use easy_auth::prelude::password::services::password_service_impl::PasswordServiceImpl;
use easy_auth::prelude::token::jwt_token_service::JwtTokenService;
use rocket::{Build, Rocket, routes};

use crate::api::components::cors::CORS;
use crate::api::routes::user_read_router::get_users;
use crate::api::routes::user_read_router::login;
use crate::api::routes::user_write_router::create_new_account;
use crate::api::services::authz_service_impl::AuthzServiceImpl;
use crate::api::services::env_service_impl::EnvServiceImpl;
use crate::api::services::user_repository_mongo::UserRepositoryMongo;
use crate::core::services::env_service::EnvService;
use crate::models::errors::custom::CustomError;

pub struct AppLauncher;

impl AppLauncher {
    pub async fn launch_rocket() -> Result<Rocket<Build>, CustomError> {

        let user_repository = UserRepositoryMongo::new()
            .await
            .map_err(|err| CustomError::new(err.to_string().as_str()))?;
        let env_service = EnvServiceImpl {};

        Ok(
            rocket::build()
                .manage(user_repository)
                .manage(JwtTokenService::new(env_service.get_secret_jwt()))
                .manage(PasswordServiceImpl::new(env_service.get_salt()))
                .manage(AuthzServiceImpl::new(env_service.get_authz_url()).await)
                .attach(CORS)
                .mount(
                    "/",
                    routes![
                            create_new_account,
                            login,
                            get_users
                        ]
                )
        )
    }
}


