use rocket::serde::json::Json;
use crate::models::views::json_data_response::JsonDataResponse;

#[get("/hello-world")]
pub async fn hello_world() -> Json<JsonDataResponse> {
    Json(
        JsonDataResponse::new("hello world 💩")
    )
}
