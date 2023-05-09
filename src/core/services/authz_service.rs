use crate::models::errors::custom::CustomError;

pub trait AuthzService {
    fn evaluate(&self, resource: &str, action: &str, pseudo: &str) -> Result<bool, CustomError>;
}