use async_trait::async_trait;
use domain::ports::polity_db_gateway::PolityDbGateway;
use tokio_postgres::Client;

pub struct PolityRepository {
    pub client: Client,
}

#[async_trait]
impl PolityDbGateway for PolityRepository {}
