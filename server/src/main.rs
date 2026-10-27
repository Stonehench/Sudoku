use std::env;

use axum::{
    Router,
    routing::{get, post},
};
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use tower_http::trace::TraceLayer;
use tracing::info;

mod entities;
mod sudokus;
mod users;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_file(true)
        .with_level(true)
        .with_line_number(true)
        .init();
    let database_url = env::var("DATABASE_URL").unwrap();

    info!("Connecting to db");
    let connection = sea_orm::Database::connect(&database_url).await.unwrap();

    info!("Connected to db");

    Migrator::up(&connection, None).await.unwrap();

    info!("Finished db migrations.");

    let state = AppState { db: connection };

    let app = Router::new()
        .route("/add_score", post(sudokus::scores::add_score))
        .route(
            "/daily",
            get(sudokus::daily::daily).post(sudokus::daily::daily),
        )
        .route("/streak", post(users::streak::streak))
        .route("/scoreboard", get(users::scoreboard::scoreboard))
        .route("/change_passwd", post(users::auth::change_password))
        .route("/login", post(users::auth::login))
        .route("/register", post(users::auth::register))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
