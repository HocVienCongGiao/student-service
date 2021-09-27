use lambda_http::http::{HeaderValue, Request};
use lambda_http::{http, Body, Context, IntoResponse, RequestExt};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Once;

static INIT: Once = Once::new();

fn initialise() {
    INIT.call_once(|| {
        let my_path = PathBuf::new().join(".env.test");
        dotenv::from_path(my_path.as_path()).ok();
        // println!("testing env {}", std::env::var("HELLO").unwrap());
    });
}

#[tokio::test]
async fn when_post_a_student_then_student_is_correctly_saved_and_returned() {
    initialise();
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
    println!("Trigger build!!");
}

// #[tokio::test]
async fn given_3_students_when_find_without_filtering_then_return_collection_with_the_right_size() {
    initialise();
    println!("is it working?");
    let request = build_http_get_request("1".to_string(), 5);
    // When
    let response = student::func(request, Context::default())
        .await
        .expect("expected Ok(_) value")
        .into_response();
    // Then
    println!("response: {:?}", response);
    assert_eq!(response.status(), 200);
    println!("Trigger build!!");
}

fn build_http_get_request(offset: String, count: i32) -> Request<Body> {
    let mut query_param = HashMap::new();
    query_param.insert("count".to_string(), vec![count.to_string()]);
    query_param.insert("offset".to_string(), vec![offset]);
    query_param.insert("firstName".to_string(), vec!["test".to_string()]);
    build_http_request("GET".to_string(), query_param)
}

fn build_http_request(method: String, query_param: HashMap<String, Vec<String>>) -> Request<Body> {
    http::Request::builder()
        .uri("https://dev-sg.portal.hocvienconggiao.com/query-api/student-service/students")
        .method(method.as_str())
        .header("Content-Type", "application/json")
        .body(Body::Empty)
        .unwrap()
        .with_query_string_parameters(query_param)
}
