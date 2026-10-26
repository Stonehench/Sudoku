use std::env;

use axum::{Router, routing::{get, post}};
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;

mod entities;
mod sudokus;
mod users;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL").unwrap();

    println!("Connecting");
    let connection = sea_orm::Database::connect(&database_url).await.unwrap();

    println!("Connected!");

    Migrator::up(&connection, None).await.unwrap();

    println!("Finished migrations.");

    let state = AppState { db: connection };

    let app = Router::new()
        .route("/daily", get(sudokus::daily::daily))
        .route("/streak", post(users::streak::streak))
        .route("/scoreboard", get(users::scoreboard::scoreboard))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    println!("Hello, world!");
}
