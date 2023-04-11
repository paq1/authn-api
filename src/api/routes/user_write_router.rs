use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;
use uuid::Uuid;
use crate::api::services::user_repository_mongo::UserRepositoryMongo;
use crate::core::entities::user::User;
use crate::core::services::user_repository::UserRepository;

use crate::models::commands::create_account_command::CreateAccountCommand;
use crate::models::views::json_data_response::JsonDataResponse;

#[post("/authn/commands/create", data="<create_command>")]
pub async fn create_new_account(
    user_repository: &State<UserRepositoryMongo>,
    create_command: Json<CreateAccountCommand>
) -> Result<Json<JsonDataResponse>, status::Custom<Json<JsonDataResponse>>> {
    let cmd = create_command.0;
    user_repository
        .insert_user(
            User {
                id: Uuid::new_v4().to_string(),
                pseudo: cmd.pseudo.clone(),
                mdp: cmd.mdp.clone()
            }
        )
        .await
        .map(|_| {
            Json(
                JsonDataResponse::new("inserted")
            )
        })
        .map_err(|err| {
            status::Custom(
                Status::BadRequest,
                Json(
                    JsonDataResponse::new(err.message.as_str())
                )
            )
        })
}