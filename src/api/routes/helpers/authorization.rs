use easy_auth::core::token::token_service::TokenService;
use easy_auth::prelude::token::jwt_token_service::JwtTokenService;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use crate::api::routes::headers::request_headers::RequestHeaders;
use crate::models::views::claims::Claims;
use crate::models::views::json_data_response::JsonDataResponse;

pub fn authenticated(
    request_headers: &RequestHeaders,
    jwt_token_service: &JwtTokenService
) -> Result<Claims, status::Custom<Json<JsonDataResponse>>> {
    request_headers
        .get_token()
        .map(|token| {

            let claims_res = jwt_token_service
                .decode::<Claims>(token.as_str());

            claims_res
                .map_err(|_| {
                    status::Custom(
                        Status::Forbidden,
                        Json(
                            JsonDataResponse::new("vous n'etes pas authentifie")
                        )
                    )
                })

        })
        .unwrap_or(
            Err(
                status::Custom(
                    Status::NetworkAuthenticationRequired,
                    Json(
                        JsonDataResponse::new("vous n'etes pas authentifié")
                    )
                )
            )
        )
}