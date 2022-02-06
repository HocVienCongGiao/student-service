use chrono::DateTime;
use hvcg_academics_openapi_student::models::{StudentSortCriteria, StudentUpsert, StudentView};
use lambda_http::http::{HeaderValue, Request};
use lambda_http::{http, Body, Context, IntoResponse, RequestExt, Response};
use std::collections::HashMap;
use std::ops::Add;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Once;
use uuid::Uuid;

pub fn build_http_request_to_post_student_upsert(student_upsert: StudentUpsert) -> Request<Body> {
    let mut query_param = HashMap::new();
    let mut path_param = HashMap::new();

    let uri = "https://dev-sg.portal.hocvienconggiao.com/mutation-api/student-service/students"
        .to_string();

    let serialized = serde_json::to_string(&student_upsert).unwrap();

    build_http_post_request(uri, query_param, path_param, Some(serialized))
}

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
    //query_param.insert("firstName".to_string(), vec!["test".to_string()]);
    query_param.insert(
        "sorts".to_string(),
        vec![StudentSortCriteria::NAME_DESC.to_string()],
    );

    let mut path_param = HashMap::new();

    let uri =
        "https://dev-sg.portal.hocvienconggiao.com/query-api/student-service/students".to_string();
    build_http_get_request(uri, query_param, path_param)
}

pub fn build_http_request_to_put_student(
    student_upsert: StudentUpsert,
    uuid: String,
) -> Request<Body> {
    let mut query_param = HashMap::new();
    let mut path_param = HashMap::new();

    let uri = format!(
        "https://dev-sg.portal.hocvienconggiao.com/mutation-api/student-service/students/{}",
        uuid
    );

    path_param.insert("id".to_string(), vec![uuid]);
    let serialized = serde_json::to_string(&student_upsert).unwrap();

    build_http_put_request(uri, query_param, path_param, Some(serialized))
}

fn build_http_get_request(
    uri: String,
    query_param: HashMap<String, Vec<String>>,
    path_param: HashMap<String, Vec<String>>,
) -> Request<Body> {
    build_http_request("GET".to_string(), uri, query_param, path_param, None)
}

fn build_http_post_request(
    uri: String,
    query_param: HashMap<String, Vec<String>>,
    path_param: HashMap<String, Vec<String>>,
    body: Option<String>,
) -> Request<Body> {
    build_http_request("POST".to_string(), uri, query_param, path_param, body)
}

fn build_http_put_request(
    uri: String,
    query_param: HashMap<String, Vec<String>>,
    path_param: HashMap<String, Vec<String>>,
    body: Option<String>,
) -> Request<Body> {
    build_http_request("PUT".to_string(), uri, query_param, path_param, body)
}

fn build_http_request(
    method: String,
    uri: String,
    query_param: HashMap<String, Vec<String>>,
    path_param: HashMap<String, Vec<String>>,
    body: Option<String>,
) -> Request<Body> {
    let mut request_body = Body::Empty;
    if let Some(body) = body {
        request_body = Body::from(body);
    }
    http::Request::builder()
        .uri(uri)
        .method(method.as_str())
        .header("Content-Type", "application/json")
        .body(request_body)
        .unwrap()
        .with_query_string_parameters(query_param)
        .with_path_parameters(path_param)
}
