use crate::db_column;
use crate::person_gateway::repository::PersonRepository;
use async_trait::async_trait;
use domain::entities::person::{Person, PersonTitle};
use domain::ports::person::find_one_person_by_id_port::FindOnePersonByIdPort;
use std::str::FromStr;
use tokio_postgres::types::ToSql;
use uuid::Uuid;

#[async_trait]
impl FindOnePersonByIdPort for PersonRepository {
    async fn find_one_by_id(&self, id: Uuid) -> Option<Person> {
        let stmt = (*self)
            .client
            .prepare("SELECT * FROM person__person_view WHERE id = $1")
            .await
            .unwrap();

        let name_param: &[&(dyn ToSql + Sync)] = &[&id];
        println!("{:?}", id.to_string());
        let row = (*self).client.query_one(&stmt, name_param).await.unwrap();

        let mut title: Option<PersonTitle> = None;
        if let Some(title_db_resp) = db_column::get_result_of_string(&row, "title") {
            title = Some(PersonTitle::from_str(&*title_db_resp).unwrap());
        }

        // TODO: check if person_id is not exist
        Some(Person {
            person_id: Some(db_column::get_uuid(&row, "id")),
            polity_id: Some(db_column::get_uuid(&row, "polity_id")),
            saint_ids: Some(db_column::get_uuid_collection(&row, "saint_ids")),
            title,
            first_name: db_column::get_result_of_string(&row, "first_name"),
            middle_name: db_column::get_result_of_string(&row, "middle_name"),
            last_name: db_column::get_result_of_string(&row, "last_name"),
            date_of_birth: Some(db_column::get_date(&row, "date_of_birth")),
            place_of_birth: db_column::get_result_of_string(&row, "place_of_birth"),
            email: db_column::get_result_of_string(&row, "email"),
            phone: db_column::get_result_of_string(&row, "phone"),
            nationality: db_column::get_result_of_string(&row, "nationality"),
            race: db_column::get_result_of_string(&row, "race"),
            personal_id_number: None, // FIXME
            address: db_column::get_result_of_string(&row, "address"),
        })
    }
}
