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

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

pub fn execute(
    status_code: u16,
    student_response: Option<StudentUpsert>,
    student_collection: Option<StudentViewCollection>,
) -> Response<Body> {
    let mut is_get_students = false;
    // let mut content_type = "application/json";
    let mut content_type: String = "application/json".to_string();
    if status_code == 204 {
        content_type = "".to_string();
        println!("status code is 204, removing application/json in Content-Type header")
    }

    let response: Response<Body> = Response::builder()
        .header(CONTENT_TYPE, content_type) // ContentType is automatically set by ApiGateway unless specifically set as an empty string
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(ACCESS_CONTROL_ALLOW_HEADERS, "*")
        .header(ACCESS_CONTROL_ALLOW_METHODS, "*")
        .status(status_code)
        .body(
            if student_response.is_none() && student_collection.is_none() {
                Body::Empty
            } else {
                if is_get_students {
                    serde_json::to_string(&student_collection)
                } else {
                    serde_json::to_string(&student_response)
                }
                .expect("unable to serialize serde_json::Value")
                .into()
            },
        )
        .expect("unable to build http::Response");
    println!(
        "final user response{:?}",
        serde_json::to_string(&student_response)
    );

    response
}

pub fn default_response(request: Request) -> Response<Body> {
    Response::builder()
        .header(CONTENT_TYPE, "application/json")
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(ACCESS_CONTROL_ALLOW_HEADERS, "*")
        .header(ACCESS_CONTROL_ALLOW_METHODS, "*")
        .status(200)
        .body(Body::Empty)
        .unwrap()
}
