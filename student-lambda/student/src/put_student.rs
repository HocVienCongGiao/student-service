// use crate::build_response;
// use crate::Error;
// use chrono::DateTime;
// use controller::StudentCollectionQuery;
// use domain::usecases::UsecaseError;
// use hvcg_academics_openapi_student::models::{
//     StudentSortCriteria, StudentUpsert, StudentViewCollection,
// };
// use jsonwebtoken::TokenData;
// use lambda_http::http::header::{
//     ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
//     CONTENT_TYPE,
// };
// use lambda_http::http::{method, uri::Uri, HeaderValue};
// use lambda_http::{handler, Body, Context, IntoResponse, Request, RequestExt, Response};
// use serde::{Deserialize, Serialize};
// use serde_json::{json, Value};
// use std::env;
// use std::str::FromStr;
//
// pub async fn execute(request: Request) -> Result<impl IntoResponse, Error> {
//     println!("Handle put method.");
//     // Update student
//     let lambda_student_request: Option<StudentUpsert> = request.payload().unwrap_or(None);
//     let student_response = controller::update_student(lambda_student_request).await;
//     build_response::execute(200, student_response, None).await
// }
