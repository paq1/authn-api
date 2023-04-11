use crate::core::entities::user::User;
use crate::models::errors::custom::CustomError;

#[async_trait]
pub trait UserRepository<Entity, Response> {
    async fn insert_user(&self, user: Entity) -> Response;
    async fn fetch_many(&self) -> Vec<Entity>;
    async fn fetch_one_by_pseudo(&self, pseudo: String) -> Result<User, CustomError>;
}