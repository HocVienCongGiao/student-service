use crate::ports::student::models::student_dbresponse::Student as StudentDbResponse;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait FindOneStudentByIdPort {
    async fn find_one_by_id(&self, id: Uuid) -> Option<StudentDbResponse>;
}
