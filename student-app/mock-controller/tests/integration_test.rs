use std::path::PathBuf;
use std::sync::Once;

static INIT: Once = Once::new();

fn initialise() {
    INIT.call_once(|| {
        let my_path = PathBuf::new().join(".env.test");
        dotenv::from_path(my_path.as_path()).ok();
        // println!("testing env {}", std::env::var("HELLO").unwrap());
    });
}

#[tokio::test]
async fn integration_works() {
    initialise();
    println!("is it working?");
    let result = 99;
    assert_eq!(result, 99);
}
