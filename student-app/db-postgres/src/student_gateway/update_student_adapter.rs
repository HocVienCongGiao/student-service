use std::ops::Add;

use crate::db_column;
use crate::student_gateway::repository::StudentRepository;

use domain::ports::student_db_gateway::{StudentDbResponse, StudentMutationDbRequest};
use domain::ports::update_student_port::UpdateStudentPort;
use domain::ports::DbError;

use async_trait::async_trait;
use chrono::NaiveDate;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Error, Transaction};
use uuid::Uuid;

pub(crate) async fn get_person_id_by_student_id(
    transaction: &Transaction<'_>,
    student_id: Uuid,
) -> Option<Uuid> {
    let stmt = (*transaction)
        .prepare("SELECT * FROM public.student__student WHERE id = $1")
        .await
        .unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&student_id];
    let result = transaction.query_one(&stmt, params).await;
    if result.is_err() {
        return None;
    }
    let row = result.unwrap();
    Some(db_column::get_uuid(&row, "person_id"))
}

#[async_trait]
impl UpdateStudentPort for StudentRepository {
    async fn find_person_id_by_student_id(&mut self, student_id: Uuid) -> Result<Uuid, DbError> {
        let mut result: Result<u64, Error>;

        let transaction = (*self)
            .client
            .transaction()
            .await
            .map_err(|error| DbError::UnknownError(error.into_source().unwrap().to_string()))?;

        let person_id = get_person_id_by_student_id(&transaction, student_id).await;
        if person_id.is_none() {
            return Err(DbError::UnknownError("get person_id fail".to_string()));
        }
        let person_id = person_id.unwrap();

        transaction
            .commit()
            .await
            .map(|_| person_id)
            .map_err(|error| DbError::UnknownError(error.into_source().unwrap().to_string()))
    }
}
