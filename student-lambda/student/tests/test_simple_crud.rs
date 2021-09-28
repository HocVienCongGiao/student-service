use chrono::DateTime;
use hvcg_academics_openapi_student::models::StudentView;
use lambda_http::http::{HeaderValue, Request};
use lambda_http::{http, Body, Context, IntoResponse, RequestExt, Response};
use std::collections::HashMap;
use std::ops::Add;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Once;
use uuid::Uuid;

mod common;
use crate::common::getter;
use common::test_data;

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
    // given_a_student_when_get_one_by_id_then_return_correct_student_view_openapi().await;
}

async fn given_a_student_when_get_one_by_id_then_return_correct_student_view_openapi() {
    // Given
    let expected_student_view_openapi: StudentView = test_data::prepare_student_view_openapi();
    let given_uuid = expected_student_view_openapi.id.to_string();

    // When
    let actual_student_view_openapi = getter::get_one_student_by_id(given_uuid).await;

    // Then
    assert_eq!(
        expected_student_view_openapi,
        actual_student_view_openapi.unwrap()
    );
}

async fn when_post_a_student_then_student_is_correctly_saved_and_returned() {
    println!("is it working?");
    // let request = build_http_get_request("1".to_string(), 5);
    // // When
    // let response = student::func(request, Context::default())
    //     .await
    //     .expect("expected Ok(_) value")
    //     .into_response();
    // // Then
    // println!("response: {:?}", response);
    // assert_eq!(response.status(), 200);
    // Init dependencies

    println!("Trigger build!!");
}

async fn given_3_students_when_find_without_filtering_then_return_collection_with_the_right_size() {
    initialise();
    // println!("is it working?");
    // let request = build_http_get_request();
    // // When
    // let response = student::func(request, Context::default())
    //     .await
    //     .expect("expected Ok(_) value")
    //     .into_response();
    // // Then
    // println!("response: {:?}", response);
    // assert_eq!(response.status(), 200);
    // println!("Trigger build!!");
}
