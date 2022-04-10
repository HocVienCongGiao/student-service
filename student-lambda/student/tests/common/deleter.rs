use crate::common::request_builder;
use lambda_http::http::StatusCode;
use lambda_http::{Context, IntoResponse};

pub async fn delete_one_student_by_id(uuid: String) -> StatusCode {
    let request = request_builder::build_http_request_to_delete_one_student(uuid);

    let response = student::func(request, Context::default())
        .await
        .expect("expected Ok(_) value")
        .into_response();

    response.status()
}
