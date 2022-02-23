use crate::helpers::spawn_app;

/// Note that each test gets run in its own runtime! To quote below
/// A second look at tokio::spawn’s documentation supports our hypothesis:
/// when a tokio runtime is shut down all tasks spawned on it are dropped.
/// actix_rt::test spins up a new runtime at the beginning of each test
/// case and they shut down at the end of each test case.

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
