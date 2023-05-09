use std::convert::Infallible;
use rocket::outcome::Outcome;
use rocket::{Request, request};
use rocket::request::FromRequest;

pub struct RequestHeaders {
    token: String
}

impl RequestHeaders {
    pub fn get_token(&self) -> Option<String> {
        if self.token.is_empty() {
            None
        } else {
            let token = self.token
                .split(" ")
                .filter(|element| element.to_lowercase() != "bearer".to_string())
                .collect::<String>();
            Some(token)
        }
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for RequestHeaders {
    type Error = Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let request_headers = request.headers();
        Outcome::Success(RequestHeaders {token: request_headers.get("Authorization").collect::<String>()})
    }
}