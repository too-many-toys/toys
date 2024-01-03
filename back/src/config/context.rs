use super::{AppState, MovieState};
use axum::Error;
use dotenv;

pub async fn load() -> Result<AppState, Error> {
  dotenv::dotenv().ok();
  tracing_subscriber::fmt::init();

  // let db_name = std::env::var("DB_NAME").unwrap();
  // let db_url = std::env::var("DB_URL").unwrap();

  let app_state = AppState {
    movie_state: MovieState {},
  };

  Ok(app_state)
}
