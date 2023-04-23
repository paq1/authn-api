use easy_auth::core::password::services::password_service::PasswordService;
use easy_auth::core::token::token_service::TokenService;
use easy_auth::prelude::password::services::password_service_impl::PasswordServiceImpl;
use easy_auth::prelude::token::jwt_token_service::JwtTokenService;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;

use crate::api::routes::headers::request_headers::RequestHeaders;
use crate::api::routes::helpers::authorization::authenticated;
use crate::api::services::user_repository_mongo::UserRepositoryMongo;
use crate::core::services::user_repository::UserRepository;
use crate::models::views::claims::Claims;
use crate::models::views::json_data_response::JsonDataResponse;
use crate::models::views::user_view::UserView;

#[get("/authn/login/<pseudo>/<mdp>")]
pub async fn login(
    user_repository: &State<UserRepositoryMongo>,
    jwt_token_service: &State<JwtTokenService>,
    password_service: &State<PasswordServiceImpl>,
    pseudo: &str,
    mdp: &str,
) -> Result<Json<JsonDataResponse>, status::Custom<Json<JsonDataResponse>>> {
    user_repository
        .fetch_one_by_pseudo(pseudo.to_string())
        .await
        .map(|user| {
            if password_service.verifie(mdp.to_string(), user.mdp.clone()) {
                let claims = Claims::new(
                    pseudo.to_string(),
                    mdp.to_string(),
                );

                let token = jwt_token_service.encode(claims);
                let message = format!("{token}");
                Ok(Json(JsonDataResponse::new(message.as_str())))
            } else {
                Err(
                    status::Custom(
                        Status::BadRequest,
                        Json(
                            JsonDataResponse::new("mdp incorrect")
                        ),
                    )
                )
            }
        })
        .unwrap_or(
            Err(
                status::Custom(
                    Status::BadRequest,
                    Json(
                        JsonDataResponse::new("login incorrect")
                    ),
                )
            )
        )
}

#[get("/authn/users")]
pub async fn get_users(
    user_repository: &State<UserRepositoryMongo>,
    jwt_token_service: &State<JwtTokenService>,
    _password_service: &State<PasswordServiceImpl>,
    request_headers: RequestHeaders,
) -> Result<Json<Vec<UserView>>, status::Custom<Json<JsonDataResponse>>> {
    match authenticated(&request_headers, &jwt_token_service) {
        Ok(_) => {
            Ok(Json(
                user_repository
                    .fetch_many()
                    .await
                    .into_iter()
                    .map(|user| {
                        UserView {
                            pseudo: user.pseudo
                        }
                    })
                    .collect::<Vec<_>>()
            ))
        }
        Err(err) => Err(err)
    }
}