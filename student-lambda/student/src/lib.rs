use std::env;
use std::str::FromStr;

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

mod build_response;
mod get_students;
mod parse_request;
mod post_student;
// pub mod delete_student;
pub mod put_student;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[derive(Deserialize, Serialize)]
struct TokenPayload {
    // Despite the struct field being named `username`, it is going to come
    // from a JSON field called `cognito:username`.
    #[serde(rename(deserialize = "cognito:username"))]
    username: String,
    #[serde(rename(deserialize = "cognito:groups"))]
    groups: Vec<String>,
}

pub async fn func(request: Request, ctx: Context) -> Result<impl IntoResponse, Error> {
    print_debug_log(&request);

    let response = match *request.method() {
        method::Method::GET => get_students::execute(request).await,
        method::Method::POST => post_student::execute(request).await,
        method::Method::PUT => put_student::execute(request).await,
        method::Method::DELETE => build_response::default_response(request), // delete_student::execute(request).await,
        _ => build_response::default_response(request),
    };

    Ok(response)
}

fn print_debug_log(request: &Request) {
    println!("Request {:?}", request);
    println!("path_parameters {:?}", request.path_parameters());
    println!(
        "query_string_parameters {:?}",
        request.query_string_parameters()
    );
    println!("Request Method {:?}", request.method());

    let default_header_value = HeaderValue::from_str(&*format!(
        "Bearer {}",
        env::var("DEFAULT_JWT_STRING").unwrap()
    ))
    .unwrap();
    let auth_header_value = request
        .headers()
        .get("authorization")
        .unwrap_or(&default_header_value);
    let auth_header_str = auth_header_value.to_str().unwrap();
    let username: String;
    let groups: Vec<String>;
    if auth_header_str != "anonymous12" {
        let jwt_token = &auth_header_str.to_string()[7..];
        let token_data: TokenData<TokenPayload> =
            jsonwebtoken::dangerous_insecure_decode(jwt_token).unwrap();
        let token_payload = token_data.claims;
        username = token_payload.username;
        groups = token_payload.groups;
        println!("Groups include {:?}", groups);
    } else {
        username = String::from("anonymous");
    }

    println!("token username {}", username);
    println!("auth_header is {}", auth_header_str);
    println!("req.headers() is {:?}", request.headers());
}
