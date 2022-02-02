use crate::ports::student::models::student_dbresponse::StudentInsert as StudentInsertDbResponse;
use crate::ports::student::models::student_mutation_dbrequest::Student as StudentMutationDbRequest;
use crate::ports::DbError;
use async_trait::async_trait;

#[async_trait]
pub trait InsertStudentPort {
    async fn insert(
        &mut self,
        db_request: StudentMutationDbRequest,
    ) -> Result<StudentInsertDbResponse, DbError>;
}
