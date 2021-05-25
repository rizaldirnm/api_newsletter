use std::net::TcpListener;

use sqlx::{PgPool};
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

// `actix_rt::test` is the testing equivalent of `actix_web::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// Use `cargo add actix-rt --dev --vers 2` to add `actix-rt`
// under `[dev-dependencies]` in Cargo.toml
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)
#[actix_rt::test]
async fn health_check_works() {
  // Arrange
  let app = spawn_app().await;

  // We need to bring in `reqwest`
  // to perform HTTP requests against our application.
  //
  // Use `cargo add reqwest --dev --vers 0.11` to add
  // it under `[dev-dependencies]` in Cargo.toml
  let client = reqwest::Client::new();

  // Act
  let response = client
    .get(&format!("{}/health_check", &app.address))
    .send()
    .await
    .expect("Failed to execute request.");

  // Assert
  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;

    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    // Act
    let response = client
      .post(&format!("{}/subscriptions", &app.address))
      .header("Content-Type", "application/x-www-form-urlencoded")
      .body(body)
      .send()
      .await
      .expect("Failed execute request");

    //Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");
  
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;


    let client = reqwest::Client::new();

    let test_case = vec![
      ("name=le%20guin", "missing the email"),
      ("email=ursula_le_guin%40gmail.com", "missing the name"),
      ("", "missing both name and email")
    ];

    for (invalid_body, err_message) in test_case {
      let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(invalid_body)
        .send()
        .await
        .expect("failed to execute request");
      
      assert_eq!(
        400,
        response.status().as_u16(),
        "The API did not fail with 400 Bad Request when the payload was {}.", err_message
      )
    }
}

pub struct TestApp {
  pub address: String,
  pub db_pool: PgPool
}

// The function is asynchronous now!
async fn spawn_app() -> TestApp {
  let listener = TcpListener::bind("127.0.0.1:0")
  .expect("Failed to bind random port");
  let port = listener.local_addr().unwrap().port();
  let address = format!("http://127.0.0.1:{}", port);
  let configuration = get_configuration().expect("Failed to read configuration.");
  let connection_pool = PgPool::connect(&configuration.database.connection_string())
  .await
  .expect("Failed to connect to Postgres.");
  
  let server = run(listener, connection_pool.clone())
  .expect("Failed to bind address");

  let _ = tokio::spawn(server);

    TestApp {
      address,
      db_pool: connection_pool,
    }
  }