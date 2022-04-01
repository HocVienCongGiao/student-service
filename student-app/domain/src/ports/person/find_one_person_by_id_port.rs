use async_trait::async_trait;
use uuid::Uuid;
use crate::entities::person::Person;

#[async_trait]
pub trait FindOnePersonByIdPort {
    async fn find_one_by_id(&self, id: Uuid) -> Option<Person>;
}
