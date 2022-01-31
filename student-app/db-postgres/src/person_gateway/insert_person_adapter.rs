use std::ops::Add;

use async_trait::async_trait;
use chrono::NaiveDate;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Error, Transaction};
use uuid::Uuid;

use domain::ports::person::insert_person_port::InsertPersonPort;
use domain::ports::person::models::person_dbresponse::Person as PersonDbResponse;
use domain::ports::person::models::person_mutation_dbrequest::Person as PersonMutationDbRequest;
use domain::ports::DbError;

use crate::person_gateway::repository::PersonRepository;

pub(crate) async fn save_id(transaction: &Transaction<'_>, id: Uuid) -> Result<u64, Error> {
    let stmt = (*transaction)
        .prepare("INSERT into public.person__person (id) VAlUES ($1)")
        .await
        .unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&id];
    transaction.execute(&stmt, params).await
}

pub(crate) async fn save_person_info(
    transaction: &Transaction<'_>,
    id: Uuid,
    field_name: String,
    value: String,
) -> Result<u64, Error> {
    let statement = format!(
        "INSERT into public.person__person_{} (id, {}) VAlUES ($1, $2)",
        field_name, field_name
    );
    let stmt = (*transaction).prepare(&statement).await.unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&id, &value];
    transaction.execute(&stmt, params).await
}

pub(crate) async fn save_personal_id_number(
    transaction: &Transaction<'_>,
    person_id: Uuid,
    personal_id_number_id: Uuid,
    personal_id_number: String,
) -> Result<u64, Error> {
    let stmt = (*transaction)
        .prepare(
            "INSERT INTO public.person__person_id_number (id, person_id, person_id_number) VAlUES ($1, $2, $3)",
        )
        .await
        .unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&personal_id_number_id, &person_id, &personal_id_number];
    transaction.execute(&stmt, params).await
}

pub(crate) async fn save_date_of_birth(
    transaction: &Transaction<'_>,
    id: Uuid,
    date_of_birth: NaiveDate,
) -> Result<u64, Error> {
    let stmt = (*transaction)
        .prepare(
            "INSERT into public.person__person_date_of_birth (id, date_of_birth) VAlUES ($1, $2)",
        )
        .await
        .unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&id, &date_of_birth];
    transaction.execute(&stmt, params).await
}

pub(crate) async fn save_date_of_issue(
    transaction: &Transaction<'_>,
    personal_id_number_id: Uuid,
    date_of_issue: NaiveDate,
) -> Result<u64, Error> {
    let stmt = (*transaction)
        .prepare(
            "INSERT INTO public.person__person_id_number_date_of_issue (id, date_of_issue) VAlUES ($1, $2)",
        )
        .await
        .unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&personal_id_number_id, &date_of_issue];
    transaction.execute(&stmt, params).await
}

pub(crate) async fn save_personal_id_number_info(
    transaction: &Transaction<'_>,
    personal_id_number_id: Uuid,
    table_name: String,
    field_name: String,
    value: String,
) -> Result<u64, Error> {
    let statement = format!(
        "INSERT INTO public.person__person_id_number_{} (id, {}) VAlUES ($1, $2)",
        table_name, field_name
    );
    let stmt = (*transaction).prepare(&statement).await.unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&personal_id_number_id, &value];
    transaction.execute(&stmt, params).await
}

pub(crate) async fn save_polity(
    transaction: &Transaction<'_>,
    person_id: Uuid,
    polity_id: Uuid,
) -> Result<u64, Error> {
    let stmt = (*transaction)
        .prepare("INSERT into public.person__person_polity (id, polity_id) VAlUES ($1, $2)")
        .await
        .unwrap();

    let params: &[&(dyn ToSql + Sync)] = &[&person_id, &polity_id];
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
    let mut ordering: i16 = 1;
    // TODO: use enumerate instead
    for christian_name in christian_names {
        let params: &[&(dyn ToSql + Sync)] = &[&id, &christian_name, &ordering];
        let stmt = (*transaction)
            .prepare("INSERT into public.person__person_christian_names (person_id, saint_id, ordering) VAlUES ($1, $2, $3)")
            .await
            .unwrap();
        result = transaction.execute(&stmt, params).await;
        ordering = ordering.add(1);
    }
    result
}
// cursor.execute("INSERT INTO ... VALUES (%s, %s)", [(1, 2), (3, 4), (5, 6)])

// pub(crate) async fn save_languages(
//     transaction: &Transaction<'_>,
//     person_id: Uuid,
//     languages: Vec<Language>,
// ) -> Result<u64, Error> {
//     // TODO: refactor this into 1 query
//     // TODO: result and then
//     let mut result: Result<u64, Error> = Ok(1_u64);
//     for language in languages {
//         let params: &[&(dyn ToSql + Sync)] = &[&person_id, &language[0], &language[1]];
//         let stmt = (*transaction)
//             .prepare("INSERT into public.person__person_language  (person_id, language, level) VAlUES ($1, $2, $3)")
//             .await
//             .unwrap();
//         result = transaction.execute(&stmt, params).await;
//     }
//     result
// }

#[async_trait]
impl InsertPersonPort for PersonRepository {
    async fn insert(
        &mut self,
        db_request: PersonMutationDbRequest,
    ) -> Result<PersonDbResponse, DbError> {
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

        result = save_person_info(&transaction, id, "title".to_string(), title.clone()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // insert vow_progress
        // let vow_progress = db_request.vow_progress.unwrap();
        //
        // result = save_person_info(
        //     &transaction,
        //     id,
        //     "vow_progress".to_string(),
        //     vow_progress.clone(),
        // )
        // .await;
        // if let Err(error) = result {
        //     return Err(DbError::UnknownError(
        //         error.into_source().unwrap().to_string(),
        //     ));
        // }

        // insert first name
        let first_name = db_request.first_name.unwrap();
        result = save_person_info(
            &transaction,
            id,
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
        result =
            save_person_info(&transaction, id, "last_name".to_string(), last_name.clone()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // insert middle name
        let middle_name = db_request.middle_name.unwrap();
        result = save_person_info(
            &transaction,
            id,
            "middle_name".to_string(),
            middle_name.clone(),
        )
        .await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // insert address
        let address = db_request.address.unwrap();
        result = save_person_info(&transaction, id, "address".to_string(), address.clone()).await;
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
        result = save_person_info(&transaction, id, "email".to_string(), email.clone()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // insert phone
        let phone = db_request.phone.unwrap();
        result = save_person_info(&transaction, id, "phone".to_string(), phone.clone()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // insert place of birth
        let place_of_birth = db_request.place_of_birth.unwrap();
        result = save_person_info(
            &transaction,
            id,
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

        // insert nationality
        let nationality = db_request.nationality.unwrap();
        result = save_person_info(
            &transaction,
            id,
            "nationality".to_string(),
            nationality.clone(),
        )
        .await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // insert race
        let race = db_request.race.unwrap();
        result = save_person_info(&transaction, id, "race".to_string(), race.clone()).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        let person_id_number = db_request.personal_id_number.unwrap();
        // insert id for personal id number
        let id_number_id = person_id_number.id.unwrap();
        let id_number = person_id_number.id_number.unwrap();
        result = save_personal_id_number(&transaction, id, id_number_id, id_number).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // insert date of issue
        let date_of_issue = person_id_number.date_of_issue.unwrap();
        result = save_date_of_issue(&transaction, id_number_id, date_of_issue).await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // insert place of issue
        let place_of_issue = person_id_number.place_of_issue.unwrap();
        result = save_personal_id_number_info(
            &transaction,
            id_number_id,
            "place_of_issue".to_string(),
            "place_of_issue".to_string(),
            place_of_issue,
        )
        .await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // insert id number provider
        let id_number_provider = person_id_number.code.unwrap();
        result = save_personal_id_number_info(
            &transaction,
            id_number_id,
            "provider".to_string(),
            "code".to_string(),
            id_number_provider.clone(),
        )
        .await;
        if let Err(error) = result {
            return Err(DbError::UnknownError(
                error.into_source().unwrap().to_string(),
            ));
        }

        // insert languages
        // let languages = db_request.languages.unwrap();
        // result = save_languages(&transaction, id, languages).await;

        transaction
            .commit()
            .await
            .map_err(|error| DbError::UnknownError(error.into_source().unwrap().to_string()));
        Ok(PersonDbResponse {
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
        })
    }
}
