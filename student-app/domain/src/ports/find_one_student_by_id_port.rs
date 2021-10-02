use crate::ports::student_db_gateway::StudentDbResponse;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait FindOneStudentByIdPort {
    async fn find_one_by_id(&self, id: Uuid) -> Option<StudentDbResponse>;
}
