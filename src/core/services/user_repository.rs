#[async_trait]
pub trait UserRepository<Entity, Response> {
    async fn insert_user(&self, user: Entity) -> Response;
    async fn fetch_many(&self) -> Vec<Entity>;
}