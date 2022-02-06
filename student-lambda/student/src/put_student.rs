use crate::build_response;
use crate::parse_request::from_request_to_id;
use chrono::DateTime;
use controller::StudentCollectionQuery;
use domain::usecases::UsecaseError;
use hvcg_academics_openapi_student::models::{
    StudentSortCriteria, StudentUpsert, StudentViewCollection,
};
use jsonwebtoken::TokenData;
use lambda_http::http::header::{
    ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
    CONTENT_TYPE,
};
use lambda_http::http::{method, uri::Uri, HeaderValue};
use lambda_http::{handler, Body, Context, IntoResponse, Request, RequestExt, Response};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;
use std::str::FromStr;
use uuid::Uuid;

pub async fn execute(request: Request) -> Response<Body> {
    println!("Handle put method.");
    let payload: Option<StudentUpsert> = request.payload().unwrap_or(None);
    if payload.is_none() {
        return build_response::execute(400, None, None);
    }

    if let Some(id) = from_request_to_id(&request) {
        put_request(payload.unwrap(), id).await
    } else {
        build_response::execute(400, None, None)
    }
}

async fn put_request(value: StudentUpsert, id: Uuid) -> Response<Body> {
    let lambda_student_request = value;
    let result = controller::update_student(lambda_student_request, id).await;
    let mut status_code = 500;
    match result {
        Ok(_) => status_code = 200,
        Err(UsecaseError::UniqueConstraintViolationError(..)) => status_code = 503,
        Err(UsecaseError::InvalidInput) => status_code = 405,
        Err(UsecaseError::ResourceNotFound) => status_code = 404,
        _ => status_code = 500,
    }

    let student_response = result.map(Some).unwrap_or_else(|e| {
        println!("error: {:?}", e);
        None
    });

    build_response::execute(status_code, student_response, None)
}
