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

pub(crate) async fn update_date_of_birth(
    transaction: &Transaction<'_>,
    id: Uuid,
    date_of_birth: NaiveDate,
) -> Result<u64, Error> {
    let stmt = (*transaction)
        .prepare("UPDATE public.person__person_date_of_birth SET date_of_birth = $2 WHERE id = $1")
        .await
        .unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&id, &date_of_birth];
    transaction.execute(&stmt, params).await
}

pub(crate) async fn update_student_info(
    transaction: &Transaction<'_>,
    id: Uuid,
    entity_name: String,
    field_name: String,
    value: String,
) -> Result<u64, Error> {
    let statement = format!(
        "UPDATE public.{}__{}_{} SET {} = $2 WHERE id = $1",
        entity_name, entity_name, field_name, field_name
    );
    let stmt = (*transaction).prepare(&statement).await.unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&id, &value];
    transaction.execute(&stmt, params).await
}

pub(crate) async fn update_polity(
    transaction: &Transaction<'_>,
    student_id: Uuid,
    polity_id: Uuid,
) -> Result<u64, Error> {
    let stmt = (*transaction)
        .prepare("UPDATE public.person__person_polity SET polity_id = $2 WHERE id = $1")
        .await
        .unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&student_id, &polity_id];
    transaction.execute(&stmt, params).await
}

pub(crate) async fn delete_christian_names(
    transaction: &Transaction<'_>,
    id: Uuid,
) -> Result<u64, Error> {
    let stmt = (*transaction)
        .prepare("DELETE FROM public.person__person_christian_names WHERE id = $1")
        .await
        .unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&id];
    transaction.execute(&stmt, params).await
}

pub(crate) async fn insert_christian_names(
    transaction: &Transaction<'_>,
    id: Uuid,
    christian_names: Vec<Uuid>,
) -> Result<u64, Error> {
    // TODO: refactor this into 1 query
    // TODO: result and then
    let mut result: Result<u64, Error> = Ok(1_u64);
    let ordering: i16 = 1;
    for christian_name in christian_names {
        let params: &[&(dyn ToSql + Sync)] = &[&id, &christian_name, &ordering];
        let stmt = (*transaction)
            .prepare("INSERT into public.person__person_christian_names (person_id, saint_id, ordering) VAlUES ($1, $2, $3)")
            .await
            .unwrap();
        result = transaction.execute(&stmt, params).await;
        ordering.add(1);
    }
    result
}

#[async_trait]
impl UpdateStudentPort for StudentRepository {
    async fn update(
        &mut self,
        db_request: StudentMutationDbRequest,
    ) -> Result<StudentDbResponse, DbError> {
        let mut result: Result<u64, Error>;

        let transaction = (*self)
            .client
            .transaction()
            .await
            .map_err(|error| DbError::UnknownError(error.into_source().unwrap().to_string()))?;

        let student_id = db_request.id.unwrap();
        let person_id = get_person_id_by_student_id(&transaction, student_id).await;
        if person_id.is_none() {
            return Err(DbError::UnknownError("get person_id fail".to_string()));
        }
        let person_id = person_id.unwrap();

        // update title
        let title = db_request.title.unwrap();
        result = update_student_info(
            &transaction,
            person_id,
            "person".to_string(),
            "title".to_string(),
            title.clone(),
        )
        .await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // update first name
        let first_name = db_request.first_name.unwrap();
        result = update_student_info(
            &transaction,
            person_id,
            "person".to_string(),
            "first_name".to_string(),
            first_name.clone(),
        )
        .await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // update last name
        let last_name = db_request.last_name.unwrap();
        result = update_student_info(
            &transaction,
            person_id,
            "person".to_string(),
            "last_name".to_string(),
            last_name.clone(),
        )
        .await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // update middle name
        let middle_name = db_request.middle_name.unwrap();
        result = update_student_info(
            &transaction,
            person_id,
            "person".to_string(),
            "middle_name".to_string(),
            last_name.clone(),
        )
        .await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // update christian names by
        // delete old value & insert new value
        let christian_names = db_request.saint_ids.unwrap();
        result = delete_christian_names(&transaction, person_id).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }
        result = insert_christian_names(&transaction, person_id, christian_names.clone()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // update date of birth
        let date_of_birth = db_request.date_of_birth.unwrap();
        result = update_date_of_birth(&transaction, person_id, date_of_birth).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // update email
        let email = db_request.email.unwrap();
        result = update_student_info(
            &transaction,
            person_id,
            "student".to_string(),
            "email".to_string(),
            email.clone(),
        )
        .await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // update phone
        let phone = db_request.phone.unwrap();
        result = update_student_info(
            &transaction,
            person_id,
            "student".to_string(),
            "phone".to_string(),
            phone.clone(),
        )
        .await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // update place of birth
        let place_of_birth = db_request.place_of_birth.unwrap();
        result = update_student_info(
            &transaction,
            person_id,
            "student".to_string(),
            "place_of_birth".to_string(),
            place_of_birth.clone(),
        )
        .await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // update polity
        let polity_id = db_request.polity_id.unwrap();
        result = update_polity(&transaction, person_id, polity_id).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        transaction
            .commit()
            .await
            .map(|_| StudentDbResponse {
                id: student_id,
                polity_id: Some(polity_id),
                saint_ids: Some(christian_names.clone()),
                title: Some(title.to_string()),
                first_name: Some(first_name.clone()),
                middle_name: Some(middle_name.clone()),
                last_name: Some(last_name.clone()),
                date_of_birth: Some(date_of_birth),
                place_of_birth: Some(place_of_birth.clone()),
                email: Some(email.clone()),
                phone: Some(phone.clone()),
                undergraduate_school: None,
            })
            .map_err(|error| DbError::UnknownError(error.into_source().unwrap().to_string()))
    }
}
