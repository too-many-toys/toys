use axum::extract::FromRef;

pub mod context;

#[derive(Clone)]
pub struct AppState {
  pub movie_state: MovieState,
}

#[derive(Clone)]
pub struct MovieState {}

impl FromRef<AppState> for MovieState {
  fn from_ref(app_state: &AppState) -> MovieState {
    app_state.movie_state.clone()
  }
}
