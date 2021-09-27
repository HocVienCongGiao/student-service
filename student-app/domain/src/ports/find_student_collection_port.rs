use crate::ports::student_db_gateway::{StudentCollectionDbResponse, StudentQueryDbRequest};
use async_trait::async_trait;

#[async_trait]
pub trait FindStudentCollectionPort {
    async fn find_collection_by(
        &self,
        db_request: StudentQueryDbRequest,
    ) -> StudentCollectionDbResponse;
}
