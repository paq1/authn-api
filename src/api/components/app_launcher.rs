use rocket::{Build, Rocket, routes};
use crate::api::components::cors::CORS;
use crate::models::errors::custom::CustomError;
use crate::api::routes::user_read_router::hello_world;
use crate::api::routes::user_write_router::create_new_account;

pub struct AppLauncher;

impl AppLauncher {
    pub async fn launch_rocket() -> Result<Rocket<Build>, CustomError> {
        Ok(
            rocket::build()
                .attach(CORS)
                .mount(
                    "/",
                    routes![
                        hello_world,
                        create_new_account
                    ]
                )
        )
    }
}


