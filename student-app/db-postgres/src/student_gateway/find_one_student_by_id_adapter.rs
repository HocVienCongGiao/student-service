use crate::db_column;
use crate::student_gateway::repository::StudentRepository;
use async_trait::async_trait;
use domain::entities::student::Student as StudentEntity;
use domain::ports::find_one_student_by_id_port::FindOneStudentByIdPort;
use tokio_postgres::types::ToSql;
use tokio_postgres::Row;
use uuid::Uuid;

#[async_trait]
impl FindOneStudentByIdPort for StudentRepository {
    async fn find_one_by_id(&self, id: Uuid) -> Option<StudentEntity> {
        let stmt = (*self)
            .client
            .prepare("SELECT * FROM student__student_view WHERE student_id = $1")
            .await
            .unwrap();

        // let stmt = block_on(stmt_future).unwrap();
        let name_param: &[&(dyn ToSql + Sync)] = &[&id];
        let row = (*self).client.query_one(&stmt, name_param).await;
        match row {
            Ok(data) => Some(from_pg_row_to_student_db_response(data)),
            Err(e) => None,
        }
    }
}

pub(crate) fn from_pg_row_to_student_db_response(row: Row) -> StudentEntity {
    StudentEntity {
        student_id: Some(db_column::get_uuid(&row, "student_id")),
        person_id: Some(db_column::get_uuid(&row, "person_id")),
    }
}
