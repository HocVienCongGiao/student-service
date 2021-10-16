use crate::ports::find_one_polity_by_id_port::FindOnePolityByIdPort;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait PolityDbGateway: FindOnePolityByIdPort {}

pub struct PolityDbResponse {
    pub id: Uuid,
    pub name: Option<String>,
    pub location_name: Option<String>,
    pub location_address: Option<String>,
    pub location_email: Option<String>,
}
