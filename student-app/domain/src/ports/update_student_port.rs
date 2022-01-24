use crate::ports::student_db_gateway::{StudentDbResponse, StudentMutationDbRequest};
use crate::ports::DbError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UpdateStudentPort {
    async fn find_person_id_by_student_id(&mut self, student_id: Uuid) -> Result<Uuid, DbError>;
}
