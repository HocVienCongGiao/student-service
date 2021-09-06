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
    let student_response = controller::get_student_by_id().await;
    let response: Response<Body> = Response::builder()
        .header(CONTENT_TYPE, "application/json")
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(ACCESS_CONTROL_ALLOW_HEADERS, "*")
        .header(ACCESS_CONTROL_ALLOW_METHODS, "*")
        .body(
            serde_json::to_string(&student_response)
                .expect("unable to serialize serde_json::Value")
                .into(),
        )
        .expect("unable to build http::Response");
    println!(
        "test1Response {:?}",
        serde_json::to_string(&student_response)
    );
    Ok(response)
    // println!("Request {:?}", request);
    // println!("path_parameters {:?}", request.path_parameters());
    // println!(
    //     "query_string_parameters {:?}",
    //     request.query_string_parameters()
    // );
    // println!("Request Method {:?}", request.method());
    //
    // if request.method() == method::Method::OPTIONS {
    //     return Ok(Response::builder()
    //         .header(CONTENT_TYPE, "application/json")
    //         .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
    //         .header(ACCESS_CONTROL_ALLOW_HEADERS, "*")
    //         .header(ACCESS_CONTROL_ALLOW_METHODS, "*")
    //         .status(200)
    //         .body(Body::Empty)
    //         .expect("unable to build http::Response"));
    // }
    //
    // let default_header_value = HeaderValue::from_str("Bearer eyJraWQiOiJaTGpneG41SStaZEpldnJRb0lpMTZEWEZoRHI4eG9UbVZ2b2ZuVm5vb3RFPSIsImFsZyI6IlJTMjU2In0.eyJzdWIiOiJmZDlhN2FmOC1mYTc2LTRiODYtYWYzZC1kOTYzNGVmNTIzNzQiLCJhdWQiOiIxcmF2NDExbmNjbnA3M2h0b3BiaG1sOHM2MSIsImNvZ25pdG86Z3JvdXBzIjpbIk9wZXJhdG9yR3JvdXAiXSwiZXZlbnRfaWQiOiI5NjQ1ZDYyMi0zZjRiLTQyYjctOWI0ZC03MWQzNWRhOTI1NmQiLCJ0b2tlbl91c2UiOiJpZCIsImF1dGhfdGltZSI6MTYyMzkzNDkyNiwiaXNzIjoiaHR0cHM6XC9cL2NvZ25pdG8taWRwLmFwLXNvdXRoZWFzdC0xLmFtYXpvbmF3cy5jb21cL2FwLXNvdXRoZWFzdC0xXzlRV1NZR3pYayIsInBob25lX251bWJlcl92ZXJpZmllZCI6dHJ1ZSwiY29nbml0bzp1c2VybmFtZSI6ImRldi1vcGVyYXRvciIsInBob25lX251bWJlciI6Iis4NDM2OTE0MDkxNiIsImV4cCI6MTYyMzk0ODAwOCwiaWF0IjoxNjIzOTQ0NDA4fQ.ml3N8J7uw4rbQOneEdnmQW6OwsAY6ycmp5PIrKGZKF3yWQn0oQECIhF2Q_jjWOjWPikpUQEy5IKgghiJLukgKo7q-T4tUauPG3GJxoSGQkfVcglkNu8nZTu7ioxXzlQAWsXLakgkH40mGzI6kl2hkEhRQh_lWGrT7TqDP2yVTsDMKEGJBdtcb-kFCnYHfn9FMoCyVGo4K3tSrkeGno7bzwO_XpFtZRhv9Qs4OtfESXARYCP3St69hyf4JuAop6-Zb38FPWcp6rnpRG3BF64YPGqo0J0MAyWVz_Du7Pk3-H5uZqqrr6iHKoPwoabPPlZxJ3JGdifVt_I54SwTbelbzw").unwrap();
    // let auth_header_value = request
    //     .headers()
    //     .get("authorization")
    //     .unwrap_or(&default_header_value);
    // let auth_header_str = auth_header_value.to_str().unwrap();
    // let username: String;
    // let groups: Vec<String>;
    // if auth_header_str != "anonymous12" {
    //     let jwt_token = &auth_header_str.to_string()[7..];
    //     let token_data: TokenData<TokenPayload> =
    //         jsonwebtoken::dangerous_insecure_decode(jwt_token).unwrap();
    //     let token_payload = token_data.claims;
    //     username = token_payload.username;
    //     groups = token_payload.groups;
    //     println!("Groups include {:?}", groups);
    // } else {
    //     username = String::from("anonymous");
    // }
    // println!("token username {}", username);
    //
    // println!("auth_header is {}", auth_header_str);
    // println!("req.headers() is {:?}", request.headers());
    // let value = json!(
    //     {
    //         "message": "Test 2 20210616 is me, how are you?"
    //     }
    // );
    //
    // Ok(Response::builder()
    //     .header(CONTENT_TYPE, "application/json")
    //     .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
    //     .header(ACCESS_CONTROL_ALLOW_HEADERS, "*")
    //     .header(ACCESS_CONTROL_ALLOW_METHODS, "*")
    //     .status(200)
    //     .body(Body::Empty)
    //     .expect("Hello student!"))
}
