use condo_plan_reporter_api::configuration::{get_configuration, DatabaseSettings};
use condo_plan_reporter_api::startup::run;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;

    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
#[actix_rt::test]
async fn create_project_returns_a_200_for_valid_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    // Act
    let response = client
        .post(&format!("{}/projects", &app.address))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "name": "rust"
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT name FROM projects",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved projects.");

    assert_eq!(saved.name, "rust");
}

#[actix_rt::test]
async fn create_project_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let invalid_json = &serde_json::json!({"name": ""});
    let error_message = "missing the name";

    // Act
    let response = client
        .post(&format!("{}/projects", &app.address))
        .header("Content-Type", "application/json")
        .json(invalid_json)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(
        400,
        response.status().as_u16(),
        // Additional customised error message on test failure
        "The API did not fail with 400 Bad Request when the payload was {}.",
        error_message
    );
}
