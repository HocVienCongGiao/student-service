use async_trait::async_trait;
use domain::ports::student::student_db_gateway::StudentDbGateway;
use tokio_postgres::Client;

pub struct StudentRepository {
    pub client: Client,
}

#[async_trait]
impl StudentDbGateway for StudentRepository {}
