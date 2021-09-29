use crate::build_response;
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

pub async fn execute(request: Request) -> Response<Body> {
    println!("Handle post method.");
    let payload: Option<StudentUpsert> = request.payload().unwrap_or(None);
    if payload.is_none() {
        return build_response::execute(400, None, None);
    }
    // Create student
    return post_request(payload.unwrap(), request).await;
}

async fn post_request(value: StudentUpsert, request: Request) -> Response<Body> {
    let lambda_student_request = value;
    let result = controller::create_student(lambda_student_request).await;
    let mut status_code = 500;
    match result {
        Ok(_) => status_code = 200,
        Err(UsecaseError::UniqueConstraintViolationError(..)) => status_code = 503,
        Err(UsecaseError::InvalidInput) => status_code = 405,
        _ => status_code = 500,
    }

    let student_response = result.map(Some).unwrap_or_else(|e| {
        println!("error: {:?}", e);
        None
    });

    build_response::execute(status_code, student_response, None)
}
