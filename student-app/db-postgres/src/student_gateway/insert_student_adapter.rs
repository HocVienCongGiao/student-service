use std::ops::Add;

use async_trait::async_trait;
use chrono::NaiveDate;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Error, Transaction};
use uuid::Uuid;

use domain::ports::insert_student_port::InsertStudentPort;
use domain::ports::student_db_gateway::{StudentDbResponse, StudentMutationDbRequest};
use domain::ports::DbError;

use crate::student_gateway::repository::StudentRepository;

pub(crate) async fn save_id(transaction: &Transaction<'_>, id: Uuid) -> Result<u64, Error> {
    let insert_person_stmt = (*transaction)
        .prepare("INSERT into public.person__person (id, type) VAlUES ($1, $2)")
        .await
        .unwrap();
    let params: &[&(dyn ToSql + Sync)] = &[&id, &"student".to_string()];
    transaction.execute(&insert_person_stmt, params).await;
    let stmt = (*transaction)
        .prepare("INSERT into public.student__person (id) VAlUES ($1)")
        .await
        .unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&id];
    // TODO: result and then
    transaction.execute(&stmt, params).await
}

pub(crate) async fn save_date_of_birth(
    transaction: &Transaction<'_>,
    id: Uuid,
    date_of_birth: NaiveDate,
) -> Result<u64, Error> {
    let stmt = (*transaction)
        .prepare(
            "INSERT into public.student__student_date_of_birth (id, date_of_birth) VAlUES ($1, $2)",
        )
        .await
        .unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&id, &date_of_birth];
    transaction.execute(&stmt, params).await
}

pub(crate) async fn save_student_info(
    transaction: &Transaction<'_>,
    id: Uuid,
    entity_name: String,
    field_name: String,
    value: String,
) -> Result<u64, Error> {
    let statement = format!(
        "INSERT into public.{}__{}_{} (id, {}) VAlUES ($1, $2)",
        entity_name, entity_name, field_name, field_name
    );
    let stmt = (*transaction).prepare(&statement).await.unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&id, &value];
    transaction.execute(&stmt, params).await
}

pub(crate) async fn save_undergraduate_school(
    transaction: &Transaction<'_>,
    student_id: Uuid,
    undergraduate_school_name: String,
) -> Result<u64, Error> {
    let stmt = (*transaction)
        .prepare("INSERT into public.student__student_undergraduate_school_name (id, school_name) VAlUES ($1, $2)")
        .await
        .unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&student_id, &undergraduate_school_name];
    transaction.execute(&stmt, params).await
}

pub(crate) async fn save_polity(
    transaction: &Transaction<'_>,
    student_id: Uuid,
    polity_id: Uuid,
) -> Result<u64, Error> {
    let stmt = (*transaction)
        .prepare("INSERT into public.person__person_polity (id, polity_id) VAlUES ($1, $2)")
        .await
        .unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&student_id, &polity_id];
    transaction.execute(&stmt, params).await
}

pub(crate) async fn save_christian_names(
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
// cursor.execute("INSERT INTO ... VALUES (%s, %s)", [(1, 2), (3, 4), (5, 6)])

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

        // insert id
        let id = db_request.id.unwrap();
        result = save_id(&transaction, id).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }
        // insert title
        let title = db_request.title.unwrap();

        result = save_student_info(
            &transaction,
            id,
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

        // insert first name
        let first_name = db_request.first_name.unwrap();
        result = save_student_info(
            &transaction,
            id,
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

        // insert last name
        let last_name = db_request.last_name.unwrap();
        result = save_student_info(
            &transaction,
            id,
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

        // insert middle name
        let middle_name = db_request.middle_name.unwrap();
        result = save_student_info(
            &transaction,
            id,
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

        // insert christian names
        let christian_names = db_request.saint_ids.unwrap();
        result = save_christian_names(&transaction, id, christian_names.clone()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // insert date of birth
        let date_of_birth = db_request.date_of_birth.unwrap();
        result = save_date_of_birth(&transaction, id, date_of_birth).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // insert email
        let email = db_request.email.unwrap();
        result = save_student_info(
            &transaction,
            id,
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

        // insert phone
        let phone = db_request.phone.unwrap();
        result = save_student_info(
            &transaction,
            id,
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

        // insert place of birth
        let place_of_birth = db_request.place_of_birth.unwrap();
        result = save_student_info(
            &transaction,
            id,
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

        // insert polity
        let polity_id = db_request.polity_id.unwrap();
        result = save_polity(&transaction, id, polity_id).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // insert undergraduate school name
        let undergraduate_school = db_request.undergraduate_school.unwrap();
        result = save_undergraduate_school(&transaction, id, undergraduate_school.clone()).await;
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
            undergraduate_school: Some(undergraduate_school.clone()),
        })
    }
}
