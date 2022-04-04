use crate::entities::student::Student as StudentEntity;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait FindOneStudentByIdPort {
    async fn find_one_by_id(&self, id: Uuid) -> Option<StudentEntity>;
}
