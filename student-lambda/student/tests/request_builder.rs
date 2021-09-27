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

pub fn build_http_request_to_get_one_student(uuid: String) -> Request<Body> {
    let mut query_param = HashMap::new();
    let mut path_param = HashMap::new();

    let uri = format!(
        "https://dev-sg.portal.hocvienconggiao.com/query-api/student-service/students/{}",
        uuid
    );

    path_param.insert("id".to_string(), vec![uuid]);
    build_http_get_request(uri, query_param, path_param)
}

pub fn build_http_request_to_get_student_collection(offset: String, count: i32) -> Request<Body> {
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
