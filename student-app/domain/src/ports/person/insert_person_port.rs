use crate::ports::person::models;
use crate::ports::DbError;
use async_trait::async_trait;
use models::person_dbresponse::Person as PersonDbResponse;
use models::person_mutation_dbrequest::Person as PersonMutationDbRequest;

#[async_trait]
pub trait InsertPersonPort {
    async fn insert(
        &mut self,
        db_request: PersonMutationDbRequest,
    ) -> Result<PersonDbResponse, DbError>;
}
