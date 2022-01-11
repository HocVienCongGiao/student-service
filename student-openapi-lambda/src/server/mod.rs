use futures::{future, future::BoxFuture, Stream, stream, future::FutureExt, stream::TryStreamExt};
use hyper::{Request, Response, StatusCode, Body, HeaderMap};
use hyper::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use log::warn;
#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::future::Future;
use std::marker::PhantomData;
use std::task::{Context, Poll};
use swagger::{ApiError, BodyExt, Has, RequestParser, XSpanIdString};
pub use swagger::auth::Authorization;
use swagger::auth::Scopes;
use url::form_urlencoded;

#[allow(unused_imports)]
use crate::models;
use crate::header;

pub use crate::context;

type ServiceFuture = BoxFuture<'static, Result<Response<Body>, crate::ServiceError>>;

use crate::{Api,
     AddStudentResponse,
     DeleteStudentResponse,
     UpdateStudentResponse,
     GetStudentByIdResponse,
     GetStudentsResponse
};

mod paths {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(vec![
            r"^/mutation-api/student-service/students$",
            r"^/mutation-api/student-service/students/(?P<id>[^/?#]*)$",
            r"^/query-api/student-service/students$",
            r"^/query-api/student-service/students/(?P<id>[^/?#]*)$"
        ])
        .expect("Unable to create global regex set");
    }
    pub(crate) static ID_MUTATION_API_STUDENT_SERVICE_STUDENTS: usize = 0;
    pub(crate) static ID_MUTATION_API_STUDENT_SERVICE_STUDENTS_ID: usize = 1;
    lazy_static! {
        pub static ref REGEX_MUTATION_API_STUDENT_SERVICE_STUDENTS_ID: regex::Regex =
            regex::Regex::new(r"^/mutation-api/student-service/students/(?P<id>[^/?#]*)$")
                .expect("Unable to create regex for MUTATION_API_STUDENT_SERVICE_STUDENTS_ID");
    }
    pub(crate) static ID_QUERY_API_STUDENT_SERVICE_STUDENTS: usize = 2;
    pub(crate) static ID_QUERY_API_STUDENT_SERVICE_STUDENTS_ID: usize = 3;
    lazy_static! {
        pub static ref REGEX_QUERY_API_STUDENT_SERVICE_STUDENTS_ID: regex::Regex =
            regex::Regex::new(r"^/query-api/student-service/students/(?P<id>[^/?#]*)$")
                .expect("Unable to create regex for QUERY_API_STUDENT_SERVICE_STUDENTS_ID");
    }
}

pub struct MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        MakeService {
            api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C, Target> hyper::service::Service<Target> for MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    type Response = Service<T, C>;
    type Error = crate::ServiceError;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, target: Target) -> Self::Future {
        futures::future::ok(Service::new(
            self.api_impl.clone(),
        ))
    }
}

fn method_not_allowed() -> Result<Response<Body>, crate::ServiceError> {
    Ok(
        Response::builder().status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .expect("Unable to create Method Not Allowed response")
    )
}

pub struct Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        Service {
            api_impl: api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C> Clone for Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    fn clone(&self) -> Self {
        Service {
            api_impl: self.api_impl.clone(),
            marker: self.marker.clone(),
        }
    }
}

impl<T, C> hyper::service::Service<(Request<Body>, C)> for Service<T, C> where
    T: Api<C> + Clone + Send + Sync + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    type Response = Response<Body>;
    type Error = crate::ServiceError;
    type Future = ServiceFuture;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.api_impl.poll_ready(cx)
    }

    fn call(&mut self, req: (Request<Body>, C)) -> Self::Future { async fn run<T, C>(mut api_impl: T, req: (Request<Body>, C)) -> Result<Response<Body>, crate::ServiceError> where
        T: Api<C> + Clone + Send + 'static,
        C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
    {
        let (request, context) = req;
        let (parts, body) = request.into_parts();
        let (method, uri, headers) = (parts.method, parts.uri, parts.headers);
        let path = paths::GLOBAL_REGEX_SET.matches(uri.path());

        match &method {

            // AddStudent - POST /mutation-api/student-service/students
            &hyper::Method::POST if path.matched(paths::ID_MUTATION_API_STUDENT_SERVICE_STUDENTS) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_student_upsert: Option<models::StudentUpsert> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_student_upsert) => param_student_upsert,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter StudentUpsert - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter StudentUpsert due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_student_upsert = match param_student_upsert {
                                    Some(param_student_upsert) => param_student_upsert,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter StudentUpsert"))
                                                        .expect("Unable to create Bad Request response for missing body parameter StudentUpsert")),
                                };

                                let result = api_impl.add_student(
                                            param_student_upsert,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddStudentResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ADD_STUDENT_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                AddStudentResponse::InvalidInput
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(405).expect("Unable to turn 405 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter StudentUpsert: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter StudentUpsert")),
                        }
            },

            // DeleteStudent - DELETE /mutation-api/student-service/students/{id}
            &hyper::Method::DELETE if path.matched(paths::ID_MUTATION_API_STUDENT_SERVICE_STUDENTS_ID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_MUTATION_API_STUDENT_SERVICE_STUDENTS_ID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MUTATION_API_STUDENT_SERVICE_STUDENTS_ID in set but failed match against \"{}\"", path, paths::REGEX_MUTATION_API_STUDENT_SERVICE_STUDENTS_ID.as_str())
                    );

                let param_id = match percent_encoding::percent_decode(path_params["id"].as_bytes()).decode_utf8() {
                    Ok(param_id) => match param_id.parse::<uuid::Uuid>() {
                        Ok(param_id) => param_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter id: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["id"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_student(
                                            param_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteStudentResponse::SuccessfulOperation
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(204).expect("Unable to turn 204 into a StatusCode");
                                                },
                                                DeleteStudentResponse::InvalidIDSupplied
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                },
                                                DeleteStudentResponse::StudentNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // UpdateStudent - PUT /mutation-api/student-service/students/{id}
            &hyper::Method::PUT if path.matched(paths::ID_MUTATION_API_STUDENT_SERVICE_STUDENTS_ID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_MUTATION_API_STUDENT_SERVICE_STUDENTS_ID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MUTATION_API_STUDENT_SERVICE_STUDENTS_ID in set but failed match against \"{}\"", path, paths::REGEX_MUTATION_API_STUDENT_SERVICE_STUDENTS_ID.as_str())
                    );

                let param_id = match percent_encoding::percent_decode(path_params["id"].as_bytes()).decode_utf8() {
                    Ok(param_id) => match param_id.parse::<uuid::Uuid>() {
                        Ok(param_id) => param_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter id: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["id"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.to_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_student_upsert: Option<models::StudentUpsert> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_student_upsert) => param_student_upsert,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter StudentUpsert - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter StudentUpsert due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_student_upsert = match param_student_upsert {
                                    Some(param_student_upsert) => param_student_upsert,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter StudentUpsert"))
                                                        .expect("Unable to create Bad Request response for missing body parameter StudentUpsert")),
                                };

                                let result = api_impl.update_student(
                                            param_id,
                                            param_student_upsert,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                UpdateStudentResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for UPDATE_STUDENT_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                UpdateStudentResponse::InvalidIDSupplied
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                },
                                                UpdateStudentResponse::StudentNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                UpdateStudentResponse::ValidationException
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(405).expect("Unable to turn 405 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter StudentUpsert: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter StudentUpsert")),
                        }
            },

            // GetStudentById - GET /query-api/student-service/students/{id}
            &hyper::Method::GET if path.matched(paths::ID_QUERY_API_STUDENT_SERVICE_STUDENTS_ID) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_QUERY_API_STUDENT_SERVICE_STUDENTS_ID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE QUERY_API_STUDENT_SERVICE_STUDENTS_ID in set but failed match against \"{}\"", path, paths::REGEX_QUERY_API_STUDENT_SERVICE_STUDENTS_ID.as_str())
                    );

                let param_id = match percent_encoding::percent_decode(path_params["id"].as_bytes()).decode_utf8() {
                    Ok(param_id) => match param_id.parse::<uuid::Uuid>() {
                        Ok(param_id) => param_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter id: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["id"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_student_by_id(
                                            param_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetStudentByIdResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_STUDENT_BY_ID_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetStudentByIdResponse::InvalidIDSupplied
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                },
                                                GetStudentByIdResponse::StudentNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetStudents - GET /query-api/student-service/students
            &hyper::Method::GET if path.matched(paths::ID_QUERY_API_STUDENT_SERVICE_STUDENTS) => {
                {
                    let authorization = match (&context as &dyn Has<Option<Authorization>>).get() {
                        &Some(ref authorization) => authorization,
                        &None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };
                }

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_name = query_params.iter().filter(|e| e.0 == "name").map(|e| e.1.to_owned())
                    .nth(0);
                let param_name = match param_name {
                    Some(param_name) => {
                        let param_name =
                            <String as std::str::FromStr>::from_str
                                (&param_name);
                        match param_name {
                            Ok(param_name) => Some(param_name),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter name - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter name")),
                        }
                    },
                    None => None,
                };
                let param_email = query_params.iter().filter(|e| e.0 == "email").map(|e| e.1.to_owned())
                    .nth(0);
                let param_email = match param_email {
                    Some(param_email) => {
                        let param_email =
                            <String as std::str::FromStr>::from_str
                                (&param_email);
                        match param_email {
                            Ok(param_email) => Some(param_email),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter email - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter email")),
                        }
                    },
                    None => None,
                };
                let param_phone = query_params.iter().filter(|e| e.0 == "phone").map(|e| e.1.to_owned())
                    .nth(0);
                let param_phone = match param_phone {
                    Some(param_phone) => {
                        let param_phone =
                            <String as std::str::FromStr>::from_str
                                (&param_phone);
                        match param_phone {
                            Ok(param_phone) => Some(param_phone),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter phone - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter phone")),
                        }
                    },
                    None => None,
                };
                let param_undergraduate_school = query_params.iter().filter(|e| e.0 == "undergraduateSchool").map(|e| e.1.to_owned())
                    .nth(0);
                let param_undergraduate_school = match param_undergraduate_school {
                    Some(param_undergraduate_school) => {
                        let param_undergraduate_school =
                            <String as std::str::FromStr>::from_str
                                (&param_undergraduate_school);
                        match param_undergraduate_school {
                            Ok(param_undergraduate_school) => Some(param_undergraduate_school),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter undergraduateSchool - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter undergraduateSchool")),
                        }
                    },
                    None => None,
                };
                let param_date_of_birth = query_params.iter().filter(|e| e.0 == "dateOfBirth").map(|e| e.1.to_owned())
                    .nth(0);
                let param_date_of_birth = match param_date_of_birth {
                    Some(param_date_of_birth) => {
                        let param_date_of_birth =
                            <chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str
                                (&param_date_of_birth);
                        match param_date_of_birth {
                            Ok(param_date_of_birth) => Some(param_date_of_birth),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter dateOfBirth - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter dateOfBirth")),
                        }
                    },
                    None => None,
                };
                let param_place_of_birth = query_params.iter().filter(|e| e.0 == "placeOfBirth").map(|e| e.1.to_owned())
                    .nth(0);
                let param_place_of_birth = match param_place_of_birth {
                    Some(param_place_of_birth) => {
                        let param_place_of_birth =
                            <String as std::str::FromStr>::from_str
                                (&param_place_of_birth);
                        match param_place_of_birth {
                            Ok(param_place_of_birth) => Some(param_place_of_birth),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter placeOfBirth - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter placeOfBirth")),
                        }
                    },
                    None => None,
                };
                let param_polity_name = query_params.iter().filter(|e| e.0 == "polityName").map(|e| e.1.to_owned())
                    .nth(0);
                let param_polity_name = match param_polity_name {
                    Some(param_polity_name) => {
                        let param_polity_name =
                            <String as std::str::FromStr>::from_str
                                (&param_polity_name);
                        match param_polity_name {
                            Ok(param_polity_name) => Some(param_polity_name),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter polityName - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter polityName")),
                        }
                    },
                    None => None,
                };
                let param_specialism = query_params.iter().filter(|e| e.0 == "specialism").map(|e| e.1.to_owned())
                    .nth(0);
                let param_specialism = match param_specialism {
                    Some(param_specialism) => {
                        let param_specialism =
                            <String as std::str::FromStr>::from_str
                                (&param_specialism);
                        match param_specialism {
                            Ok(param_specialism) => Some(param_specialism),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter specialism - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter specialism")),
                        }
                    },
                    None => None,
                };
                let param_sorts = query_params.iter().filter(|e| e.0 == "sorts").map(|e| e.1.to_owned())
                    .filter_map(|param_sorts| param_sorts.parse().ok())
                    .collect::<Vec<_>>();
                let param_sorts = if !param_sorts.is_empty() {
                    Some(param_sorts)
                } else {
                    None
                };
                let param_offset = query_params.iter().filter(|e| e.0 == "offset").map(|e| e.1.to_owned())
                    .nth(0);
                let param_offset = match param_offset {
                    Some(param_offset) => {
                        let param_offset =
                            <i32 as std::str::FromStr>::from_str
                                (&param_offset);
                        match param_offset {
                            Ok(param_offset) => Some(param_offset),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter offset - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter offset")),
                        }
                    },
                    None => None,
                };
                let param_count = query_params.iter().filter(|e| e.0 == "count").map(|e| e.1.to_owned())
                    .nth(0);
                let param_count = match param_count {
                    Some(param_count) => {
                        let param_count =
                            <i32 as std::str::FromStr>::from_str
                                (&param_count);
                        match param_count {
                            Ok(param_count) => Some(param_count),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter count - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter count")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.get_students(
                                            param_name,
                                            param_email,
                                            param_phone,
                                            param_undergraduate_school,
                                            param_date_of_birth,
                                            param_place_of_birth,
                                            param_polity_name,
                                            param_specialism,
                                            param_sorts.as_ref(),
                                            param_offset,
                                            param_count,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetStudentsResponse::SuccessfulOperation
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_STUDENTS_SUCCESSFUL_OPERATION"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetStudentsResponse::InvalidIDSupplied
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                },
                                                GetStudentsResponse::StudentNotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            _ if path.matched(paths::ID_MUTATION_API_STUDENT_SERVICE_STUDENTS) => method_not_allowed(),
            _ if path.matched(paths::ID_MUTATION_API_STUDENT_SERVICE_STUDENTS_ID) => method_not_allowed(),
            _ if path.matched(paths::ID_QUERY_API_STUDENT_SERVICE_STUDENTS) => method_not_allowed(),
            _ if path.matched(paths::ID_QUERY_API_STUDENT_SERVICE_STUDENTS_ID) => method_not_allowed(),
            _ => Ok(Response::builder().status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .expect("Unable to create Not Found response"))
        }
    } Box::pin(run(self.api_impl.clone(), req)) }
}

/// Request parser for `Api`.
pub struct ApiRequestParser;
impl<T> RequestParser<T> for ApiRequestParser {
    fn parse_operation_id(request: &Request<T>) -> Result<&'static str, ()> {
        let path = paths::GLOBAL_REGEX_SET.matches(request.uri().path());
        match request.method() {
            // AddStudent - POST /mutation-api/student-service/students
            &hyper::Method::POST if path.matched(paths::ID_MUTATION_API_STUDENT_SERVICE_STUDENTS) => Ok("AddStudent"),
            // DeleteStudent - DELETE /mutation-api/student-service/students/{id}
            &hyper::Method::DELETE if path.matched(paths::ID_MUTATION_API_STUDENT_SERVICE_STUDENTS_ID) => Ok("DeleteStudent"),
            // UpdateStudent - PUT /mutation-api/student-service/students/{id}
            &hyper::Method::PUT if path.matched(paths::ID_MUTATION_API_STUDENT_SERVICE_STUDENTS_ID) => Ok("UpdateStudent"),
            // GetStudentById - GET /query-api/student-service/students/{id}
            &hyper::Method::GET if path.matched(paths::ID_QUERY_API_STUDENT_SERVICE_STUDENTS_ID) => Ok("GetStudentById"),
            // GetStudents - GET /query-api/student-service/students
            &hyper::Method::GET if path.matched(paths::ID_QUERY_API_STUDENT_SERVICE_STUDENTS) => Ok("GetStudents"),
            _ => Err(()),
        }
    }
}
