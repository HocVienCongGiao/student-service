use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Once;

use lambda_http::http::header::{
    ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
    CONTENT_TYPE,
};
use lambda_http::http::HeaderValue;
use lambda_http::{
    handler, http, lambda_runtime, Body, Context, IntoResponse, Request, RequestExt, Response,
};

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

static INIT: Once = Once::new();

fn initialise() {
    INIT.call_once(|| {
        let my_path = PathBuf::new().join(".env.test");
        dotenv::from_path(my_path.as_path()).ok();
        // println!("testing env {}", std::env::var("HELLO").unwrap());
    });
}

#[tokio::test]
async fn crud() {
    initialise();
    println!("is it working?");
    let mut query_param = HashMap::new();
    query_param.insert("count".to_string(), vec!["5".to_string()]);
    query_param.insert("offset".to_string(), vec!["1".to_string()]);
    query_param.insert("firstName".to_string(), vec!["test".to_string()]);
    let request = http::Request::builder()
        .uri("https://dev-sg.portal.hocvienconggiao.com/query-api/student-service/users?offset=1&count=5")
        .method("GET")
        .header("Content-Type", "application/json")
        .body(Body::Empty)
        .unwrap()
        .with_query_string_parameters(query_param);

    // When
    let response = student::func(request, Context::default())
        .await
        .expect("expected Ok(_) value")
        .into_response();
    // Then
    println!("response: {:?}", response);
    assert_eq!(response.status(), 200);
    println!("Trigger build!");
}
