use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

use crate::models::commands::create_account_command::CreateAccountCommand;
use crate::models::views::json_data_response::JsonDataResponse;

#[post("/authn/commands/create", data="<create_command>")]
pub async fn create_new_account(
    create_command: Json<CreateAccountCommand>
) -> Result<Json<JsonDataResponse>, status::Custom<Json<JsonDataResponse>>> {

    let _cmd = create_command.0;

    Ok(Json(JsonDataResponse::new("account not created for the moment")))
}