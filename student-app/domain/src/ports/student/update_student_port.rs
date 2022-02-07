use crate::ports::student::models::student_dbresponse::Student as StudentDbResponse;
use crate::ports::student::models::student_mutation_dbrequest::Student as StudentMutationDbRequest;
use crate::ports::DbError;
use async_trait::async_trait;

#[async_trait]
pub trait UpdateStudentPort {
    async fn update(
        &mut self,
        db_request: StudentMutationDbRequest,
    ) -> Result<StudentDbResponse, DbError>;
}
