use chrono::DateTime;
use controller::StudentCollectionQuery;
use domain::boundaries::usecase_boundary::UsecaseError;
use hvcg_academics_openapi_student::models::{Student, StudentSortCriteria, StudentViewCollection};
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
    if request.method() == method::Method::OPTIONS {
        return Ok(Response::builder()
            .header(CONTENT_TYPE, "application/json")
            .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .header(ACCESS_CONTROL_ALLOW_HEADERS, "*")
            .header(ACCESS_CONTROL_ALLOW_METHODS, "*")
            .status(200)
            .body(Body::Empty)
            .expect("unable to build http::Response"));
    }
    let status_code: u16;
    let student_response: Option<Student>;
    let student_collection: Option<StudentViewCollection>;
    let mut is_get_students = false;

    match *request.method() {
        method::Method::GET => {
            println!("Handle get method.");
            if let Some(id) = get_id_from_request(&request) {
                // get student by id
                student_response = controller::get_student_by_id(id).await;
                student_collection = None;
                status_code = 200
            } else {
                // get students
                let query = get_query_from_request(&request);
                student_collection = Some(controller::get_students(query).await);
                is_get_students = true;
                student_response = None;
                status_code = 200;
            }
        }
        method::Method::POST => {
            println!("Handle post method.");
            student_collection = None;
            // Create student
            if let Some(value) = request.payload().unwrap_or(None) {
                let lambda_student_request = value;
                let result = controller::create_student(&lambda_student_request).await;
                match result {
                    Ok(_) => status_code = 200,
                    Err(UsecaseError::UniqueConstraintViolationError(..)) => status_code = 503,
                    Err(UsecaseError::InvalidInput) => status_code = 405,
                    _ => status_code = 500,
                }
                student_response = result.map(Some).unwrap_or_else(|e| {
                    println!("error: {:?}", e);
                    None
                });
            } else {
                student_response = None;
                status_code = 400;
            }
        }
        method::Method::PUT => {
            println!("Handle put method.");
            // Update student
            let lambda_student_request: Option<Student> = request.payload().unwrap_or(None);
            student_response = controller::update_student(lambda_student_request).await;
            student_collection = None;
            status_code = 200;
        }
        method::Method::DELETE => {
            println!("Handle delete method.");
            // Delete student
            status_code = 204;
            student_response = None;
            student_collection = None;
        }
        _ => {
            student_collection = None;
            student_response = None;
            status_code = 404
        }
    }

    let mut content_type = "application/json";
    if status_code == 204 {
        content_type = "";
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

    Ok(response)
}

pub fn get_id_from_request(req: &Request) -> Option<uuid::Uuid> {
    let path_parameters = req.path_parameters();
    let id_param = path_parameters.get("id");
    if let Some(id) = id_param {
        println!("id found");
        Some(uuid::Uuid::parse_str(id).unwrap())
    } else {
        println!("id not found");
        None
    }
}

pub fn get_query_from_request(req: &Request) -> StudentCollectionQuery {
    let query = req.query_string_parameters();
    let param_date_of_birth = query.get("date_of_birth");
    let param_date_of_birth = match param_date_of_birth {
        Some(param_date_of_birth) => {
            let param_date_of_birth =
                <DateTime<chrono::Utc> as FromStr>::from_str(param_date_of_birth);
            match param_date_of_birth {
                Ok(param_date_of_birth) => Some(param_date_of_birth),
                Err(e) => {
                    println!("{:?}", e);
                    None
                }
            }
        }
        None => None,
    };

    let sort_criteria_dto: Vec<String> = query
        .get_all("sorts")
        .unwrap_or_default()
        .iter()
        .map(|e| e.to_string())
        .collect();

    let mut param_sorts: Option<Vec<StudentSortCriteria>> = None;

    if !sort_criteria_dto.is_empty() {
        let mut sort_criteria = Vec::new();
        sort_criteria_dto.iter().for_each(|criterion| {
            let s: StudentSortCriteria = criterion.parse().unwrap();
            sort_criteria.push(s);
        });
        param_sorts = Option::from(sort_criteria);
    }

    StudentCollectionQuery {
        name: from_query_param_to_string(req, "name"),
        email: from_query_param_to_string(req, "email"),
        phone: from_query_param_to_string(req, "phone"),
        undergraduate_school: from_query_param_to_string(req, "undergraduate_school"),
        date_of_birth: param_date_of_birth,
        place_of_birth: from_query_param_to_string(req, "place_of_birth"),
        polity_name: from_query_param_to_string(req, "polity_name"),
        specialism: from_query_param_to_string(req, "specialism"),
        sorts: param_sorts,
        offset: query.get("offset").map(|str| str.parse().unwrap()),
        count: query.get("count").map(|str| str.parse().unwrap()),
    }
}

fn from_query_param_to_string(request: &Request, param: &str) -> Option<String> {
    let query = request.query_string_parameters();
    query.get(param).map(|str| str.parse().unwrap())
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
