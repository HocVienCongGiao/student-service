use crate::ports::person_db_gateway::{PersonDbResponse, PersonMutationDbRequest};
use crate::ports::DbError;
use async_trait::async_trait;

#[async_trait]
pub trait UpdatePersonPort {
    async fn update(
        &mut self,
        db_request: PersonMutationDbRequest,
    ) -> Result<PersonDbResponse, DbError>;
}
