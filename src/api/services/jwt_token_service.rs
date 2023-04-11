use jsonwebtoken::{EncodingKey, Header};
use crate::core::services::token_services::TokenService;
use crate::models::views::claims::Claims;

pub struct JwtTokenService {
    secret_key: String
}

impl JwtTokenService {
    pub fn new() -> Self {
        Self {
            secret_key: std::env::var("SECRET_ENCODING_KEY")
                .expect("pas de secret kkey")
        }
    }
}

impl TokenService for JwtTokenService {
    fn encode(&self, claims: Claims) -> String {
        let secret_key = EncodingKey::from_secret(self.secret_key.as_bytes());
        let header = Header::new(jsonwebtoken::Algorithm::HS256);
        jsonwebtoken::encode(&header, &claims, &secret_key).unwrap()
    }
}