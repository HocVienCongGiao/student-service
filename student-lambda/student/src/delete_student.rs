use crate::build_response;
use crate::parse_request::from_request_to_id;
use crate::Error;
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
use lambda_http::http::{method, uri::Uri, HeaderValue, StatusCode};
use lambda_http::{handler, Body, Context, IntoResponse, Request, RequestExt, Response};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;
use std::str::FromStr;

pub async fn execute(request: Request) -> Response<Body> {
    println!("Handle delete method");
    let status_code = match from_request_to_id(&request) {
        Some(id) => {
            controller::delete_one_student_by_id(id).await;
            StatusCode::NO_CONTENT
        }
        None => {
            StatusCode::NOT_FOUND
            // TODO: UsecaseError::ResourceNotFound
            // Ok(())
        } // _ => StatusCode::INTERNAL_SERVER_ERROR
    };
    build_response::execute(status_code.as_u16(), None, None)
}
