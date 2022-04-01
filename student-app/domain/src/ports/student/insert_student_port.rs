use crate::ports::DbError;
use async_trait::async_trait;
use crate::entities::student::Student;

#[async_trait]
pub trait InsertStudentPort {
    async fn insert(
        &mut self,
        db_request: Student,
    ) -> Result<Student, DbError>;
}
