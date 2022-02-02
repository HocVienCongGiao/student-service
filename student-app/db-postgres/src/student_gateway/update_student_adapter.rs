use crate::student_gateway::repository::StudentRepository;
use async_trait::async_trait;
use domain::ports::student::models::student_dbresponse::Student as StudentDbResponse;
use domain::ports::student::models::student_mutation_dbrequest::Student as StudentMutationDbRequest;
use domain::ports::student::update_student_port::UpdateStudentPort;
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
