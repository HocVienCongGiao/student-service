use chrono::DateTime;
use common::getter;
use common::poster;
use common::test_data;
use hvcg_academics_openapi_student::models::{StudentUpsert, StudentView, StudentViewCollection};
use lambda_http::{Body, Context, http, IntoResponse, RequestExt, Response};
use lambda_http::http::{HeaderValue, Request};
use std::collections::HashMap;
use std::ops::Add;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Once;
use uuid::Uuid;

use crate::common::poster::post_student_upsert;

mod common;

static INIT: Once = Once::new();

fn initialise() {
    INIT.call_once(|| {
        let my_path = PathBuf::new().join(".env.test");
        dotenv::from_path(my_path.as_path()).ok();
        // println!("testing env {}", std::env::var("HELLO").unwrap());
    });
}

#[tokio::test]
async fn crud_should_work() {
    initialise();
    given_a_student_when_get_one_by_id_then_return_correct_student_view_openapi().await;
    when_post_a_student_upsert_then_student_is_correctly_saved_and_student_view_returned().await;
    // given_3_students_when_find_without_filtering_then_return_collection_with_the_right_size().await;
    test_get_collection().await;
}

async fn given_a_student_when_get_one_by_id_then_return_correct_student_view_openapi() {
    // Given
    let expected_student_view_openapi: StudentView = test_data::prepare_student_view_openapi(None);
    let given_uuid = expected_student_view_openapi.id.to_string();

    // When
    let actual_student_view_openapi = getter::get_one_student_by_id(given_uuid).await;

    // Then
    assert_eq!(
        expected_student_view_openapi,
        actual_student_view_openapi.unwrap()
    );
}

async fn when_post_a_student_upsert_then_student_is_correctly_saved_and_student_view_returned() {
    // Given
    let given_student_upsert_openapi: StudentUpsert = test_data::prepare_student_upsert_openapi();

    // When
    let actual_student_view_openapi =
        poster::post_student_upsert(given_student_upsert_openapi).await;

    // Then
    // assert_eq!(actual_student_view_openapi.is_some());
    let mut actual_id: Option<Uuid> = actual_student_view_openapi.clone().map(|t| t.id);
    let expected_student_view_openapi = test_data::prepare_student_view_openapi(actual_id);
    assert_eq!(
        expected_student_view_openapi,
        actual_student_view_openapi.unwrap()
    );
}

async fn given_3_students_when_find_without_filtering_then_return_collection_with_the_right_size() {
    todo!()
}

async fn test_get_collection() {
    let expected_student_view_collection_openapi: StudentViewCollection =
        test_data::prepare_student_view_collection_openapi();

    let actual_student_view_collection_openapi = getter::get_student_collection().await;

    assert!(!actual_student_view_collection_openapi
        .unwrap()
        .students
        .is_empty());
    // assert_eq!(
    //     expected_student_view_collection_openapi,
    //     actual_student_view_collection_openapi.unwrap()
    // );
}
