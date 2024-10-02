use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Deserialize, Serialize, Default)]
pub struct Movie {
    id: String,
    name: String,
    year: u16,
    was_good: bool,
}

struct AppState {
    db: HashMap<String, Movie>,
}

#[tokio::main]
async fn main() {
    // Create Axum server with the following endpoints:
    // 1. GET /movie/{id} - This should return back a movie given the id
    // 2. POST /movie - this should save movie in a DB (HashMap<String, Movie>). This movie will be sent
    // via a JSON payload.

    // As a bonus: implement a caching layer so we don't need to make expensive "DB" lookups, etc.
    let shared_db = Arc::new(tokio::sync::RwLock::new(AppState { db: HashMap::new() }));

    let app = Router::new()
        // 1. GET /movie/{id} - This should return back a movie given the id
        .route("/movie/:movie_id", get(get_movie_handler))
        // 2. POST /movie - this should save movie in a DB (HashMap<String, Movie>). This movie will be sent
        .route("/movie", post(create_movie_handler))
        .with_state(shared_db);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn get_movie_handler(
    Path(movie_id): Path<String>,
    State(db): State<Arc<tokio::sync::RwLock<AppState>>>,
) -> Result<impl IntoResponse, StatusCode> {
    if let Some(movie_by_id) = db.read().await.db.get(&movie_id) {
        return Ok(Json(movie_by_id.clone()));
    }
    Err(StatusCode::NOT_FOUND)
}

async fn create_movie_handler(
    State(db): State<Arc<tokio::sync::RwLock<AppState>>>,
    Json(payload): Json<Movie>,
) -> impl IntoResponse {
    match db.write().await.db.entry(payload.id.clone()) {
        std::collections::hash_map::Entry::Occupied(_) => StatusCode::CONFLICT,
        std::collections::hash_map::Entry::Vacant(entry) => {
            entry.insert(payload.to_owned());
            StatusCode::CREATED
        }
    }
}
