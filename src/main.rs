mod auth;
mod db;

use auth::register;
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {
    pub db: Pool<Sqlite>,
}

const DB_URL: &str = "sqlite:info.db";

#[tokio::main]
async fn main() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Database doesn't exist, creating...");

        Sqlite::create_database(DB_URL).await.unwrap();
    }
    let state = AppState {
        db: SqlitePool::connect("sqlite:info.db").await.unwrap(),
    };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/register", post(register))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
