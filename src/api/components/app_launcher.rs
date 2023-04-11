use rocket::{Build, Rocket, routes};
use crate::api::components::cors::CORS;
use crate::models::errors::custom::CustomError;
use crate::api::routes::user_read_router::hello_world;
use crate::api::routes::user_read_router::login;
use crate::api::routes::user_write_router::create_new_account;
use crate::api::services::jwt_token_service::JwtTokenService;
use crate::api::services::user_repository_mongo::UserRepositoryMongo;

pub struct AppLauncher;

impl AppLauncher {
    pub async fn launch_rocket() -> Result<Rocket<Build>, CustomError> {
        UserRepositoryMongo::new().await
            .map(|user_repository| {
                rocket::build()
                    .manage(user_repository)
                    .manage(JwtTokenService::new())
                    .attach(CORS)
                    .mount(
                        "/",
                        routes![
                            hello_world,
                            create_new_account,
                            login
                        ]
                    )
            })
            .map_err(|err| CustomError::new(err.to_string().as_str()))
    }
}


