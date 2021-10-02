use crate::db_column;
use crate::polity_gateway::repository::PolityRepository;
use async_trait::async_trait;
use domain::ports::find_one_polity_by_id_port::FindOnePolityByIdPort;
use domain::ports::polity_db_gateway::PolityDbResponse;
use tokio_postgres::types::ToSql;
use uuid::Uuid;

#[async_trait]
impl FindOnePolityByIdPort for PolityRepository {
    async fn find_one_by_id(&self, id: Uuid) -> Option<PolityDbResponse> {
        let stmt = (*self)
            .client
            .prepare("SELECT * FROM polity__polity_view WHERE id = $1")
            .await
            .unwrap();

        // let stmt = block_on(stmt_future).unwrap();
        let name_param: &[&(dyn ToSql + Sync)] = &[&id];
        let row = (*self).client.query_one(&stmt, name_param).await.unwrap();

        Some(PolityDbResponse {
            id,
            name: Some(db_column::get_string(&row, "name")),
            location_name: Some(db_column::get_string(&row, "location_name")),
            location_address: Some(db_column::get_string(&row, "location_address")),
            location_email: Some(db_column::get_string(&row, "location_email")),
        })
    }
}
