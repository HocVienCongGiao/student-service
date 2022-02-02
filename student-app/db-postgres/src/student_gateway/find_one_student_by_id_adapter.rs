use crate::db_column;
use crate::student_gateway::repository::StudentRepository;
use async_trait::async_trait;
use domain::ports::find_one_student_by_id_port::FindOneStudentByIdPort;
use domain::ports::student::models::student_dbresponse::Student as StudentDbResponse;
use tokio_postgres::types::ToSql;
use tokio_postgres::Row;
use uuid::Uuid;

#[async_trait]
impl FindOneStudentByIdPort for StudentRepository {
    async fn find_one_by_id(&self, id: Uuid) -> Option<StudentDbResponse> {
        let stmt = (*self)
            .client
            .prepare("SELECT * FROM student__student_view WHERE student_id = $1")
            .await
            .unwrap();

        // let stmt = block_on(stmt_future).unwrap();
        let name_param: &[&(dyn ToSql + Sync)] = &[&id];
        let row = (*self).client.query_one(&stmt, name_param).await.unwrap();
        Some(from_pg_row_to_student_db_response(row))
    }
}

pub(crate) fn from_pg_row_to_student_db_response(row: Row) -> StudentDbResponse {
    StudentDbResponse {
        id: db_column::get_uuid(&row, "student_id"),
        polity_id: Some(db_column::get_uuid(&row, "polity_id")),
        saint_ids: Some(db_column::get_uuid_collection(&row, "saint_ids")),
        title: Some(db_column::get_string(&row, "title")),
        first_name: Some(db_column::get_string(&row, "first_name")),
        middle_name: Some(db_column::get_string(&row, "middle_name")),
        last_name: Some(db_column::get_string(&row, "last_name")),
        date_of_birth: Some(db_column::get_date(&row, "date_of_birth")),
        place_of_birth: Some(db_column::get_string(&row, "place_of_birth")),
        email: Some(db_column::get_string(&row, "email")),
        phone: Some(db_column::get_string(&row, "phone")),
        // TODO: refactor this (undergraduate school: educational stage(level = bachelor)
        undergraduate_school: None,
    }
}
