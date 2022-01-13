use crate::student_gateway::repository::StudentRepository;
use async_trait::async_trait;
use domain::ports::student_db_gateway::{StudentDbResponse, StudentMutationDbRequest};
use domain::ports::update_student_port::UpdateStudentPort;
use domain::ports::DbError;

#[async_trait]
impl UpdateStudentPort for StudentRepository {
    async fn update(
        &mut self,
        db_request: StudentMutationDbRequest,
    ) -> Result<StudentDbResponse, DbError> {
        todo!()
    }
}
