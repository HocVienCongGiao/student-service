use crate::student_gateway::repository::StudentRepository;
use async_trait::async_trait;
use domain::ports::find_one_student_by_id::FindOneStudentById;
use domain::ports::student_db_gateway::StudentDbResponse;
use uuid::Uuid;

#[async_trait]
impl FindOneStudentById for StudentRepository {
    async fn find_one_by_id(&self, id: Uuid) -> Option<StudentDbResponse> {
        todo!()
    }
}
