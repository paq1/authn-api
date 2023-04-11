use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::State;
use crate::api::services::jwt_token_service::JwtTokenService;
use crate::api::services::user_repository_mongo::UserRepositoryMongo;
use crate::core::services::token_services::TokenService;
use crate::core::services::user_repository::UserRepository;
use crate::models::views::claims::Claims;
use crate::models::views::json_data_response::JsonDataResponse;

#[get("/hello-world")]
pub async fn hello_world() -> Json<JsonDataResponse> {
    Json(
        JsonDataResponse::new("hello world 💩")
    )
}

#[get("/authn/login/<pseudo>/<mdp>")]
pub async fn login(
    user_repository: &State<UserRepositoryMongo>,
    jwt_token_service: &State<JwtTokenService>,
    pseudo: &str,
    mdp: &str
) -> Result<Json<JsonDataResponse>, status::Custom<Json<JsonDataResponse>>> {
    user_repository
        .fetch_one_by_pseudo(pseudo.to_string())
        .await
        .map(|user| {
            if user.mdp.as_str() == mdp {

                let claims = Claims::new(
                    pseudo.to_string(),
                    mdp.to_string()
                );

                let token = jwt_token_service.encode(claims);
                let message = format!("token : {token}");
                Ok(Json(JsonDataResponse::new(message.as_str())))
            } else {
                Err(
                    status::Custom(
                        Status::BadRequest,
                        Json(
                            JsonDataResponse::new("mdp incorrect")
                        )
                    )
                )
            }
        })
        .unwrap_or(
            Err(
                status::Custom(
                    Status::BadRequest,
                    Json(
                        JsonDataResponse::new("mdp incorrect")
                    )
                )
            )
        )
        .map_err(|err| {
            status::Custom(
                Status::BadRequest,
                Json(
                    JsonDataResponse::new("erreur login")
                )
            )
        })
}