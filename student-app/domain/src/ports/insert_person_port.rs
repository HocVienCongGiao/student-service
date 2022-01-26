use crate::ports::person_db_gateway::{PersonDbResponse, PersonMutationDbRequest};
use crate::ports::DbError;
use async_trait::async_trait;

#[async_trait]
pub trait InsertPersonPort {
    async fn insert(
        &mut self,
        db_request: PersonMutationDbRequest,
        // TODO: review StudentDbResponse
    ) -> Result<PersonDbResponse, DbError>;
}
