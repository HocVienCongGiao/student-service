use crate::build_response;
use crate::parse_request::{from_request_to_collection_query, from_request_to_id};
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
    println!("Handle get method.");
    let response: Response<Body>;

    return if let Some(id) = from_request_to_id(&request) {
        get_student_by_id(id).await
    } else {
        get_students(request).await
    };
}

async fn get_student_by_id(id: Uuid) -> Response<Body> {
    let student_response = controller::get_one_student_by_id(id).await;
    build_response::execute(200, student_response, None)
}

async fn get_students(request: Request) -> Response<Body> {
    let query = from_request_to_collection_query(&request);
    let student_collection = Some(controller::get_students(query).await);
    build_response::execute(200, None, student_collection)
}
