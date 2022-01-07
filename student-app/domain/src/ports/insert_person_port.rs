use crate::ports::person_db_gateway::{
    EducationalStageMutationDbRequest, PersonDbResponse, PersonMutationDbRequest,
};
use crate::ports::DbError;
use async_trait::async_trait;

#[async_trait]
pub trait InsertPersonPort {
    async fn insert(
        &mut self,
        db_request: PersonMutationDbRequest,
    ) -> Result<PersonDbResponse, DbError>;

    async fn insert_educational_stage(
        &mut self,
        db_request: EducationalStageMutationDbRequest,
    ) -> Result<EducationalStageDbResponse, DbError>;
}
