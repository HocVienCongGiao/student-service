use crate::ports::student_db_gateway::{StudentDbResponse, StudentMutationDbRequest};
use crate::ports::DbError;
use async_trait::async_trait;

#[async_trait]
pub trait InsertStudentPort {
    async fn insert(
        &mut self,
        db_request: StudentMutationDbRequest,
    ) -> Result<StudentDbResponse, DbError>;
}
