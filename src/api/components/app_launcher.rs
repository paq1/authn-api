use auth_lib_paq1::prelude::password::services::password_service_impl::PasswordServiceImpl;
use rocket::{Build, Rocket, routes};
use crate::api::components::cors::CORS;
use crate::models::errors::custom::CustomError;
use crate::api::routes::user_read_router::login;
use crate::api::routes::user_write_router::create_new_account;
use crate::api::services::env_service_impl::EnvServiceImpl;
use crate::api::services::jwt_token_service::JwtTokenService;
use crate::api::services::user_repository_mongo::UserRepositoryMongo;
use crate::core::services::env_service::EnvService;

pub struct AppLauncher;

impl AppLauncher {
    pub async fn launch_rocket() -> Result<Rocket<Build>, CustomError> {
        UserRepositoryMongo::new().await
            .map(|user_repository| {

                let env_service = Box::new(EnvServiceImpl {});

                rocket::build()
                    .manage(user_repository)
                    .manage(JwtTokenService::new())
                    .manage(PasswordServiceImpl::new(env_service.get_salt()))
                    .attach(CORS)
                    .mount(
                        "/",
                        routes![
                            create_new_account,
                            login
                        ]
                    )
            })
            .map_err(|err| CustomError::new(err.to_string().as_str()))
    }
}


