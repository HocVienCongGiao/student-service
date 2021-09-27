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
    let expected_student_view_openapi: StudentView = prepare_student_view_openapi();
    let given_uuid = expected_student_view_openapi.id.unwrap().to_string();

    // When
    let actual_student_view_openapi = get_one_student_by_id(given_uuid).await;

    // Then
    assert_eq!(
        expected_student_view_openapi,
        actual_student_view_openapi.unwrap()
    );
}

async fn get_one_student_by_id(uuid: String) -> Option<StudentView> {
    let request = build_http_request_to_get_one_student(uuid);
    // When
    let response = student::func(request, Context::default())
        .await
        .expect("expected Ok(_) value")
        .into_response();

    let mut student_view_openapi: Option<StudentView> = None;
    if let Body::Text(body) = response.body() {
        student_view_openapi =
            Some(serde_json::from_str(body).expect("Unable deserialise response body"));
    }
    student_view_openapi
}

fn prepare_student_view_openapi() -> StudentView {
    StudentView {
        id: Some(Uuid::from_str("53f549b9-99bf-4e12-88e3-c2f868953283").unwrap()), // TODO make required
        polity_name: Some("Cần Thơ".to_string()),
        polity_location_name: Some("Tòa Giám Mục Cần Thơ".to_string()),
        polity_location_address: Some("12 Nguyễn Trãi, Ninh Kiều, Cần Thơ".to_string()),
        polity_location_email: Some("binh@sunrise.vn".to_string()),
        christian_name: Some("Phêrô".to_string()),
        title: Some("PRIEST".to_string()),
        name: Some("Nguyễn Hữu Chiến".to_string()),
        date_of_birth: Some(DateTime::from_str("1983-05-16 00:00:00+00:00").unwrap()),
        place_of_birth: Some("Tra Vinh".to_string()),
        email: Some("binh@sunrise.vn".to_string()),
        phone: Some("+84 1228019700".to_string()),
        undergraduate_school: Some("Đại Chủng Viện Thánh Quý - Cần Thơ".to_string()),
        specialism: None, // TODO remove
    }
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

// #[tokio::test]
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

fn build_http_request_to_get_one_student(uuid: String) -> Request<Body> {
    let mut query_param = HashMap::new();
    let mut path_param = HashMap::new();

    let uri = format!(
        "https://dev-sg.portal.hocvienconggiao.com/query-api/student-service/students/{}",
        uuid
    );

    path_param.insert("id".to_string(), vec![uuid]);
    build_http_get_request(uri, query_param, path_param)
}

fn build_http_request_to_get_student_collection(offset: String, count: i32) -> Request<Body> {
    let mut query_param = HashMap::new();
    query_param.insert("count".to_string(), vec![count.to_string()]);
    query_param.insert("offset".to_string(), vec![offset]);
    query_param.insert("firstName".to_string(), vec!["test".to_string()]);

    let mut path_param = HashMap::new();

    let uri =
        "https://dev-sg.portal.hocvienconggiao.com/query-api/student-service/students".to_string();
    build_http_get_request(uri, query_param, path_param)
}

fn build_http_get_request(
    uri: String,
    query_param: HashMap<String, Vec<String>>,
    path_param: HashMap<String, Vec<String>>,
) -> Request<Body> {
    build_http_request("GET".to_string(), uri, query_param, path_param)
}

fn build_http_request(
    method: String,
    uri: String,
    query_param: HashMap<String, Vec<String>>,
    path_param: HashMap<String, Vec<String>>,
) -> Request<Body> {
    http::Request::builder()
        .uri(uri)
        .method(method.as_str())
        .header("Content-Type", "application/json")
        .body(Body::Empty)
        .unwrap()
        .with_query_string_parameters(query_param)
        .with_path_parameters(path_param)
}
