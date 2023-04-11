use crate::models::views::claims::Claims;

pub trait TokenService {
    fn encode(&self, claims: Claims) -> String;
}
