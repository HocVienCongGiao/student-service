use async_trait::async_trait;
use domain::ports::person_db_gateway::{
    PersonDbGateway, PersonDbResponse, PersonMutationDbRequest,
};
use tokio_postgres::{Client, Error, Row};

pub struct PersonRepository {
    pub client: Client,
}

#[async_trait]
impl PersonDbGateway for PersonRepository {}
