use axum::{
  http::{HeaderMap, Request},
  middleware::{self, Next},
  response::Response,
  routing::{get, post},
  Router,
};

use crate::config::AppState;

// pub fn user_routes() -> Router<AppState> {
//     Router::new()
//       // Multipart
//       .route("/collection", post(user::put_my_collection))
// }
