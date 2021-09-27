use async_trait::async_trait;
use domain::ports::student_db_gateway::{
    StudentDbGateway, StudentDbResponse, StudentMutationDbRequest, StudentQueryDbRequest,
};
use tokio_postgres::{Client, Error, Row};

pub struct StudentRepository {
    pub client: Client,
}

#[async_trait]
impl StudentDbGateway for StudentRepository {}
