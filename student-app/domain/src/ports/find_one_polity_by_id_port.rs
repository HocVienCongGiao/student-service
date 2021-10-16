use crate::ports::polity_db_gateway::PolityDbResponse;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait FindOnePolityByIdPort {
    async fn find_one_by_id(&self, id: Uuid) -> Option<PolityDbResponse>;
}
