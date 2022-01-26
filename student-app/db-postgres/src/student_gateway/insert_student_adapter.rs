use async_trait::async_trait;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Error, Transaction};
use uuid::Uuid;

use domain::ports::insert_student_port::InsertStudentPort;
use domain::ports::student_db_gateway::{StudentInsertDbResponse, StudentMutationDbRequest};
use domain::ports::DbError;

use crate::student_gateway::repository::StudentRepository;

pub(crate) async fn save_student_id(
    transaction: &Transaction<'_>,
    student_id: Uuid,
    person_id: Uuid,
) -> Result<u64, Error> {
    let stmt = (*transaction)
        .prepare("INSERT into public.student__student (id, person_id) VAlUES ($1, $2)")
        .await
        .unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&student_id, &person_id];
    transaction.execute(&stmt, params).await
}

#[async_trait]
impl InsertStudentPort for StudentRepository {
    async fn insert(
        &mut self,
        db_request: StudentMutationDbRequest,
    ) -> Result<StudentInsertDbResponse, DbError> {
        let result: Result<u64, Error>;

        let transaction = (*self)
            .client
            .transaction()
            .await
            .map_err(|error| DbError::UnknownError(error.into_source().unwrap().to_string()))?;

        // insert student_id
        let person_id = db_request.person_id.unwrap();
        let student_id = db_request.student_id.unwrap();
        result = save_student_id(&transaction, student_id, person_id).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        transaction
            .commit()
            .await
            .map_err(|error| DbError::UnknownError(error.into_source().unwrap().to_string()));

        Ok(StudentInsertDbResponse {
            student_id,
            person_id,
        })
    }
}
