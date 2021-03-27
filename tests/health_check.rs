use condo_plan_reporter_api::run;
use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
// No .await call, therefore no need for`spawn_app`to be async now.
// We are also running tests, so it is not worth it to propagate errors:// if we fail to perform the required setup we can just panic and crash
// all the things.
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address.");
    // Launch the server as a background task
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
