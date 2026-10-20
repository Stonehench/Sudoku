use std::env;

use axum::{Router, routing::get};
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;

mod entities;
mod sudokus;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL").unwrap();
    let connection = sea_orm::Database::connect(&database_url).await.unwrap();

    Migrator::up(&connection, None).await.unwrap();

    let state = AppState { db: connection };

    let app = Router::new()
        .route("/daily", get(sudokus::daily::daily))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    println!("Hello, world!");
}
