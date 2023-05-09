use easy_auth::core::password::services::password_service::PasswordService;
use easy_auth::prelude::password::services::password_service_impl::PasswordServiceImpl;
use easy_auth::prelude::token::jwt_token_service::JwtTokenService;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;
use uuid::Uuid;
use crate::api::routes::headers::request_headers::RequestHeaders;
use crate::api::routes::helpers::authorization::authorised;
use crate::api::services::authz_service_impl::AuthzServiceImpl;
use crate::api::services::user_repository_mongo::UserRepositoryMongo;
use crate::core::entities::user::User;
use crate::core::services::user_repository::UserRepository;

use crate::models::commands::create_account_command::CreateAccountCommand;
use crate::models::views::json_data_response::JsonDataResponse;

use rocket::futures::TryFutureExt;

#[post("/authn/commands/create", data="<create_command>")]
pub async fn create_new_account(
    user_repository: &State<UserRepositoryMongo>,
    password_service: &State<PasswordServiceImpl>,
    jwt_token_service: &State<JwtTokenService>,
    authz_service: &State<AuthzServiceImpl>,
    request_headers: RequestHeaders,
    create_command: Json<CreateAccountCommand>
) -> Result<Json<JsonDataResponse>, status::Custom<Json<JsonDataResponse>>> {

    async {
        authorised(
            &request_headers,
            jwt_token_service,
            authz_service,
            "urn:api:authn",
            "create"
        )
    }
        .and_then(|_| {
            create_without_authz(
                user_repository,
                password_service,
                create_command
            )
        })
        .await
}

async fn create_without_authz(
    user_repository: &State<UserRepositoryMongo>,
    password_service: &State<PasswordServiceImpl>,
    create_command: Json<CreateAccountCommand>
) -> Result<Json<JsonDataResponse>, status::Custom<Json<JsonDataResponse>>> {
    let cmd = create_command.0;
    user_repository
        .insert_user(
            User {
                id: Uuid::new_v4().to_string(),
                pseudo: cmd.pseudo.clone(),
                mdp: password_service.create_hash_password(cmd.mdp.clone())
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
