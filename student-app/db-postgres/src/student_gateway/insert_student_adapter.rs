use std::ops::Add;

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use tokio_postgres::types::ToSql;
use tokio_postgres::{Error, Transaction};
use uuid::Uuid;

use domain::ports::insert_student_port::InsertStudentPort;
use domain::ports::student_db_gateway::{StudentDbResponse, StudentMutationDbRequest};
use domain::ports::DbError;

use crate::student_gateway::repository::StudentRepository;

pub(crate) async fn save_id(
    transaction: &Transaction<'_>,
    id: Uuid,
    person_id: Uuid,
) -> Result<u64, Error> {
    let stmt = (*transaction)
        .prepare("INSERT into public.student__student (id, person_id) VAlUES ($1, $2)")
        .await
        .unwrap();
    let params: &[&(dyn ToSql + Sync)] = &[&id, &person_id];
    transaction.execute(&stmt, params).await
}

#[async_trait]
impl InsertStudentPort for StudentRepository {
    async fn insert(
        &mut self,
        db_request: StudentMutationDbRequest,
    ) -> Result<StudentDbResponse, DbError> {
        let mut result: Result<u64, Error>;

        let transaction = (*self)
            .client
            .transaction()
            .await
            .map_err(|error| DbError::UnknownError(error.into_source().unwrap().to_string()))?;

        // insert student_id
        let id = db_request.id.unwrap();
        let person_id = db_request.person_id.unwrap();
        result = save_id(&transaction, id, person_id).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        transaction
            .commit()
            .await
            .map_err(|error| DbError::UnknownError(error.into_source().unwrap().to_string()));
        Ok(StudentDbResponse {
            id,
            polity_id: None,
            saint_ids: None,
            title: None,
            first_name: None,
            middle_name: None,
            last_name: None,
            date_of_birth: None,
            place_of_birth: None,
            email: None,
            phone: None,
            // person_id: Some(person_id),
            undergraduate_school: None,
        })
    }
}
