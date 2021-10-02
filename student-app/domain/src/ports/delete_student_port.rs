use crate::ports::DbError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait DeleteStudentPort {
    async fn delete(&mut self, id: Uuid) -> Result<(), DbError>;
}
