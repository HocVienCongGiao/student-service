use crate::student_gateway::repository::StudentRepository;
use async_trait::async_trait;
use domain::ports::delete_student_port::DeleteStudentPort;
// use domain::ports::student_db_gateway::{StudentDbResponse, StudentMutationDbRequest};
use domain::ports::DbError;
use uuid::Uuid;

#[async_trait]
impl DeleteStudentPort for StudentRepository {
    async fn delete(&mut self, id: Uuid) -> Result<(), DbError> {
        todo!()
    }
}
