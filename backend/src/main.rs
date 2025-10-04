mod handlers;
mod models;
mod routes;
mod schemas;

use std::sync::Arc;

use axum::{Json, Router, response::IntoResponse, routing::get};
use dotenv::dotenv;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub struct AppState {
    db: Pool<Postgres>,
}

async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Simple CRUD API with Rust, SQLX, Postgres,and Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let app_state = Arc::new(AppState { db: pool.clone() });
    let app = Router::new()
        .route("/api/v1/healthcheck", get(health_checker_handler))
        .nest(
            "/api/v1/archives",
            routes::archives::get_routes(app_state.clone()),
        )
        .nest(
            "/api/v1/documents",
            routes::documents::get_routes(app_state.clone()),
        )
        .nest(
            "/api/v1/institutes",
            routes::institutes::get_routes(app_state.clone()),
        )
        .nest(
            "/api/v1/places",
            routes::places::get_routes(app_state.clone()),
        );

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string()) // fallback for local dev
        .parse()
        .expect("PORT must be a number");

    println!("🚀 Server starting on port {}", port);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
