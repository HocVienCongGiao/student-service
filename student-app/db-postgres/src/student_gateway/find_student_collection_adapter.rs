use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use heck::SnakeCase;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Client, Error, Row};

use domain::ports::find_student_collection_port::FindStudentCollectionPort;
use domain::ports::student_db_gateway::{
    StudentCollectionDbResponse, StudentDbResponse, StudentQueryDbRequest,
    StudentSortCriteriaDbRequest, StudentSortFieldDbRequest,
};
use domain::SortDirection;

use crate::student_gateway::find_one_student_by_id_adapter::from_pg_row_to_student_db_response;
use crate::student_gateway::repository::StudentRepository;

pub struct StudentFilter {
    name: String,
    email: String,
    phone: String,
    date_of_birth: Option<NaiveDate>,
    place_of_birth: String,
    polity_name: String,
}

#[async_trait]
impl FindStudentCollectionPort for StudentRepository {
    async fn find_collection_by(
        &self,
        db_request: StudentQueryDbRequest,
    ) -> StudentCollectionDbResponse {
        let name = db_request.name.unwrap_or_else(|| "".to_string());
        let email = db_request.email.unwrap_or_else(|| "".to_string());
        let phone = db_request.phone.unwrap_or_else(|| "".to_string());
        let date_of_birth = db_request.date_of_birth.map(|date_time| date_time);
        let place_of_birth = db_request.place_of_birth.unwrap_or_else(|| "".to_string());
        let polity_name = db_request.polity_name.unwrap_or_else(|| "".to_string());
        let offset = db_request.offset.unwrap_or(0);
        let count = db_request.count.unwrap_or(20);

        let order_by_string: String;
        if let Some(sort_db_request) = db_request.sort_request {
            order_by_string = build_order_string(sort_db_request.sort_criteria);
        } else {
            order_by_string = default_order_string();
        }

        let filtering_string = build_filtering_query_statement_string();

        let student_filter = StudentFilter {
            name,
            email,
            phone,
            date_of_birth,
            place_of_birth,
            polity_name,
        };

        let result = find_by(
            &(*self).client,
            &student_filter,
            count,
            offset,
            filtering_string.clone(),
            order_by_string.clone(),
        )
        .await;
        let collection: Vec<StudentDbResponse>;
        if let Ok(result) = result {
            collection = result
                .into_iter()
                .map(from_pg_row_to_student_db_response) //fn in find one by id
                .collect();
        } else {
            collection = vec![];
        }

        let has_more: Option<bool>;
        let total_from_offset = count_without_limit(
            &(*self).client,
            &student_filter,
            offset,
            filtering_string.clone(),
            order_by_string,
        )
        .await
        .unwrap();
        if total_from_offset > count {
            has_more = Some(true);
        } else {
            has_more = Some(false);
        }
        let total = count_total(&(*self).client, &student_filter, filtering_string)
            .await
            .unwrap();
        StudentCollectionDbResponse {
            collection,
            has_more,
            total,
        }
    }
}

fn to_query_string(sort_criteria: &StudentSortCriteriaDbRequest) -> String {
    let field_str = &*sort_criteria.field.to_string();
    let field_str_sc = field_str.to_snake_case();
    format!(
        "{} {}",
        field_str_sc.to_lowercase(),
        sort_criteria.direction.to_string()
    )
}

fn build_order_string(vec_sort_criteria: Vec<StudentSortCriteriaDbRequest>) -> String {
    let mut order_by_strings: Vec<String> = Vec::new();
    for (_i, e) in vec_sort_criteria.iter().enumerate() {
        order_by_strings.push(to_query_string(e));
    }
    order_by_strings.join(", ")
}

fn default_order_string() -> String {
    let vec_sort_criteria = vec![
        StudentSortCriteriaDbRequest {
            field: StudentSortFieldDbRequest::LastName,
            direction: SortDirection::Asc,
        },
        StudentSortCriteriaDbRequest {
            field: StudentSortFieldDbRequest::MiddleName,
            direction: SortDirection::Asc,
        },
        StudentSortCriteriaDbRequest {
            field: StudentSortFieldDbRequest::FirstName,
            direction: SortDirection::Asc,
        },
    ];
    build_order_string(vec_sort_criteria)
}

fn build_filtering_query_statement_string() -> String {
    "(unaccent(last_name) LIKE ('%' || unaccent($1) || '%') \
        OR unaccent(middle_name) LIKE ('%' || unaccent($1) || '%')  \
        OR unaccent(first_name) LIKE ('%' || unaccent($1) || '%')) \
        AND (unaccent(email) LIKE ('%' || unaccent($2) || '%') OR email is NULL) \
        AND (unaccent(phone) LIKE ('%' || unaccent($3) || '%') OR phone is NULL) \
        AND (unaccent(undergraduate_school_name) LIKE ('%' || unaccent($4) || '%') OR undergraduate_school_name is NULL) \
        AND ($5::DATE is null OR date_of_birth = $5::DATE) \
        AND (unaccent(place_of_birth) LIKE ('%' || unaccent($6) || '%') OR place_of_birth is NULL) \
        AND (unaccent(polity_name) LIKE ('%' || unaccent($7) || '%') OR polity_name is NULL) \
        "
        .to_string()
}

async fn find_by(
    client: &Client,
    filter: &StudentFilter,
    count: i64,
    offset: i64,
    filtering_string: String,
    order_by_string: String,
) -> Result<Vec<Row>, Error> {
    let statement = format!(
        "SELECT * FROM student__student_view \
        WHERE {} \
        ORDER BY {} \
        LIMIT $8 OFFSET $9",
        filtering_string, order_by_string
    );

    println!("statement = {}", statement);
    let stmt = (*client).prepare(&statement).await.unwrap();
    let name_param: &[&(dyn ToSql + Sync)] = &[
        &filter.name,
        &filter.email,
        &filter.phone,
        &filter.date_of_birth,
        &filter.place_of_birth,
        &filter.polity_name,
        &count,
        &offset,
    ];
    client.query(&stmt, name_param).await
}

pub async fn count_without_limit(
    client: &Client,
    filter: &StudentFilter,
    offset: i64,
    filtering_string: String,
    order_by_string: String,
) -> Result<i64, Error> {
    let statement = format!(
        "SELECT COUNT(*) FROM
        (SELECT * FROM student__student_view \
        WHERE {} \
        ORDER BY {} \
        LIMIT ALL OFFSET $8) AS students",
        filtering_string, order_by_string
    );

    println!("statement = {}", statement);
    let stmt = (*client).prepare(&statement).await.unwrap();
    let name_param: &[&(dyn ToSql + Sync)] = &[
        &filter.name,
        &filter.email,
        &filter.phone,
        &filter.date_of_birth,
        &filter.place_of_birth,
        &filter.polity_name,
        &offset,
    ];
    Ok(client.query_one(&stmt, name_param).await?.get("count"))
}

pub async fn count_total(
    client: &Client,
    filter: &StudentFilter,
    filtering_string: String,
) -> Result<i64, Error> {
    let statement = format!(
        "SELECT COUNT(*) FROM student__student_view \
        WHERE {}",
        filtering_string
    );

    println!("statement = {}", statement);
    let stmt = (*client).prepare(&statement).await.unwrap();
    let name_param: &[&(dyn ToSql + Sync)] = &[
        &filter.name,
        &filter.email,
        &filter.phone,
        &filter.date_of_birth,
        &filter.place_of_birth,
        &filter.polity_name,
    ];
    Ok(client.query_one(&stmt, name_param).await?.get("count"))
}
