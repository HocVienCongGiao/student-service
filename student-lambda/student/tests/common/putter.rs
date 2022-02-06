use crate::common::request_builder;
use hvcg_academics_openapi_student::models::{StudentUpsert, StudentView};
use lambda_http::{http, Body, Context, IntoResponse, RequestExt, Response};

pub async fn put_student(student_upsert: StudentUpsert, uuid: String) -> Option<StudentView> {
    let request = request_builder::build_http_request_to_put_student(student_upsert, uuid);
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