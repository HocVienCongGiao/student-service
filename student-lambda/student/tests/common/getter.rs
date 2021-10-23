use crate::common::request_builder;
use hvcg_academics_openapi_student::models::{StudentView, StudentViewCollection};
use lambda_http::{http, Body, Context, IntoResponse, RequestExt, Response};

pub async fn get_one_student_by_id(uuid: String) -> Option<StudentView> {
    let request = request_builder::build_http_request_to_get_one_student(uuid);
    // When
    let response = student::func(request, Context::default())
        .await
        .expect("expected Ok(_) value")
        .into_response();

    let mut student_view_openapi: Option<StudentView> = None;
    if let Body::Text(body) = response.body() {
        student_view_openapi =
            Some(serde_json::from_str(body).expect("Unable to deserialise response body"));
    }
    student_view_openapi
}

pub async fn get_student_collection() -> Option<StudentViewCollection> {
    let request = request_builder::build_http_request_to_get_student_collection(0.to_string(), 10);
    // When
    let response = student::func(request, Context::default())
        .await
        .expect("expected Ok(_) value")
        .into_response();

    let mut student_view_collection_openapi: Option<StudentViewCollection> = None;
    if let Body::Text(body) = response.body() {
        student_view_collection_openapi =
            Some(serde_json::from_str(body).expect("Unable to deserialise response body"));
    }
    student_view_collection_openapi
}
