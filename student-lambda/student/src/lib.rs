use jsonwebtoken::TokenData;
use lambda_http::http::header::{
    ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
    CONTENT_TYPE,
};
use lambda_http::http::{method, uri::Uri, HeaderValue};
use lambda_http::{handler, Body, Context, IntoResponse, Request, RequestExt, Response};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;
use hvcg_student_openapi_student::models::{Student, StudentCollection};
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
    println!("Request {:?}", request);
    println!("path_parameters {:?}", request.path_parameters());
    println!(
        "query_string_parameters {:?}",
        request.query_string_parameters()
    );
    println!("Request Method {:?}", request.method());
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

    let default_header_value = HeaderValue::from_str("Bearer eyJraWQiOiJaTGpneG41SStaZEpldnJRb0lpMTZEWEZoRHI4eG9UbVZ2b2ZuVm5vb3RFPSIsImFsZyI6IlJTMjU2In0.eyJzdWIiOiJmZDlhN2FmOC1mYTc2LTRiODYtYWYzZC1kOTYzNGVmNTIzNzQiLCJhdWQiOiIxcmF2NDExbmNjbnA3M2h0b3BiaG1sOHM2MSIsImNvZ25pdG86Z3JvdXBzIjpbIk9wZXJhdG9yR3JvdXAiXSwiZXZlbnRfaWQiOiI5NjQ1ZDYyMi0zZjRiLTQyYjctOWI0ZC03MWQzNWRhOTI1NmQiLCJ0b2tlbl91c2UiOiJpZCIsImF1dGhfdGltZSI6MTYyMzkzNDkyNiwiaXNzIjoiaHR0cHM6XC9cL2NvZ25pdG8taWRwLmFwLXNvdXRoZWFzdC0xLmFtYXpvbmF3cy5jb21cL2FwLXNvdXRoZWFzdC0xXzlRV1NZR3pYayIsInBob25lX251bWJlcl92ZXJpZmllZCI6dHJ1ZSwiY29nbml0bzp1c2VybmFtZSI6ImRldi1vcGVyYXRvciIsInBob25lX251bWJlciI6Iis4NDM2OTE0MDkxNiIsImV4cCI6MTYyMzk0ODAwOCwiaWF0IjoxNjIzOTQ0NDA4fQ.ml3N8J7uw4rbQOneEdnmQW6OwsAY6ycmp5PIrKGZKF3yWQn0oQECIhF2Q_jjWOjWPikpUQEy5IKgghiJLukgKo7q-T4tUauPG3GJxoSGQkfVcglkNu8nZTu7ioxXzlQAWsXLakgkH40mGzI6kl2hkEhRQh_lWGrT7TqDP2yVTsDMKEGJBdtcb-kFCnYHfn9FMoCyVGo4K3tSrkeGno7bzwO_XpFtZRhv9Qs4OtfESXARYCP3St69hyf4JuAop6-Zb38FPWcp6rnpRG3BF64YPGqo0J0MAyWVz_Du7Pk3-H5uZqqrr6iHKoPwoabPPlZxJ3JGdifVt_I54SwTbelbzw").unwrap();

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
    let status_code: u16;
    let student_response: Option<Student>;
    let student_collection: Option<StudentCollection>;
    let mut is_get_students = false;

    match *request.method() {
        method::Method::GET => {
            println!("Handle get method.");
            if let Some(id) = get_id_from_request(&request) {
                // get student by id
                student_response = controller::get_student_by_id().await;
                student_collection = None;
                status_code = 200
            } else {
                // get students
                let query = get_query_from_request(&request);
                let first_name: Option<String> = query.first_name;
                let offset: Option<u16> = query.offset;
                let count: Option<u16> = query.count;
                student_collection = Some(controller::get_students().await);
                is_get_students = true;
                student_response = None;
                status_code = 200;
            }
        }
        method::Method::POST => {
            println!("Handle post method.");
            // Create student
            student_response = controller::create_student().await;
            student_collection = None;
            status_code = 200;
        }
        method::Method::PUT => {
            println!("Handle put method.");
            // Update student
            student_response = controller::update_student().await;
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
    }

    let response: Response<Body> = Response::builder()
        .header(CONTENT_TYPE, "application/json")
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

pub fn get_query_from_request(req: &Request) -> StudentQuery {
    let query = req.query_string_parameters();
    StudentQuery {
        first_name: query.get("firstName").map(|str| str.to_string()),
        offset: query.get("offset").map(|str| str.parse().unwrap()),
        count: query.get("count").map(|str| str.parse().unwrap()),
    }
}

pub struct StudentQuery {
    first_name: Option<String>,
    offset: Option<u16>,
    count: Option<u16>,
}
