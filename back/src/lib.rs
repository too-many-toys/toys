use std::net::SocketAddr;

use anyhow::Context;
use axum::{
  http::header::{AUTHORIZATION, CONTENT_TYPE},
  http::{HeaderValue, StatusCode},
  routing::{get, post},
  Json, Router,
};
use tower_http::cors::{Any, CorsLayer};

pub mod config;
pub mod errors;
pub mod model;
pub mod service;

pub async fn start_server() -> Result<(), anyhow::Error> {
  let app_state = config::context::load().await.unwrap();

  let app = Router::new()
    .route("/health", get("OK"))
    .route("/json", post(get_json))
    .layer(
      CorsLayer::new()
        .allow_origin(Any) // TODO 배포 시 변경
        .allow_methods(Any)
        .allow_headers(vec![CONTENT_TYPE, AUTHORIZATION]),
    )
    .with_state(app_state);

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();

  axum::serve(listener, app.into_make_service())
    .with_graceful_shutdown(shutdown_signal())
    .await
    .context("HTTP server error")?;

  Ok(())
}

async fn shutdown_signal() {
  tokio::signal::ctrl_c()
    .await
    .expect("Expect shutdown signal handler");
  println!("Server Down")
}

async fn get_json(Json(payload): Json<A>) -> (StatusCode, &'static str) {
  tracing::info!("asc");
  tracing::info!("payload: {:?}", payload);
  (StatusCode::CREATED, "1231231")
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct A {
  name: String,
  description: String,
  image: String,
  attributes: Vec<Attribute>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Attribute {
  trait_type: String,
  value: String,
}
