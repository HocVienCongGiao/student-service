use crate::db_column;
use crate::student_gateway::repository::StudentRepository;
use async_trait::async_trait;
use domain::ports::delete_student_port::DeleteStudentPort;
use domain::ports::DbError;
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

pub(crate) async fn delete_person(transaction: &Transaction<'_>, id: Uuid) -> Result<u64, Error> {
    let stmt = (*transaction)
        .prepare("DELETE FROM public.person__person WHERE id = $1")
        .await
        .unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&id];
    transaction.execute(&stmt, params).await
}

#[async_trait]
impl DeleteStudentPort for StudentRepository {
    async fn delete(&mut self, id: Uuid) -> Result<(), DbError> {
        let result: Result<u64, Error>;

        let transaction = (*self)
            .client
            .transaction()
            .await
            .map_err(|error| DbError::UnknownError(error.into_source().unwrap().to_string()))?;

        let student_id = id;
        let person_id = get_person_id_by_student_id(&transaction, student_id).await;
        if person_id.is_none() {
            return Err(DbError::UnknownError("get person_id fail".to_string()));
        }
        let person_id = person_id.unwrap();

        result = delete_person(&transaction, person_id).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        transaction
            .commit()
            .await
            .map(|_| ())
            .map_err(|error| DbError::UnknownError(error.into_source().unwrap().to_string()))
    }
}
