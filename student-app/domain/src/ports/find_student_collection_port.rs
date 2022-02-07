use crate::ports::student::models::student_dbrequest::StudentQuery as StudentQueryDbRequest;
use crate::ports::student::models::student_dbresponse::StudentCollection as StudentCollectionDbResponse;
use async_trait::async_trait;

#[async_trait]
pub trait FindStudentCollectionPort {
    async fn find_collection_by(
        &self,
        db_request: StudentQueryDbRequest,
    ) -> StudentCollectionDbResponse;
}
