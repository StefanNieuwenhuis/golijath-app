mod db;
mod handlers;
mod models;
mod routes;
mod schemas;

use axum::{
    Router,
    http::{
        HeaderValue, Method,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    },
};
use dotenv::dotenv;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

use db::AppState;

pub fn create_app(app_state: Arc<AppState>) -> Router {
    let frontend_origin =
        std::env::var("FRONTEND_ORIGIN").unwrap_or_else(|_| "http://localhost:5173".to_string());

    let cors = CorsLayer::new()
        .allow_origin(frontend_origin.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    return Router::new()
        .nest("/api/v1/healthcheck", routes::health_check::get_routes())
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
        )
        .layer(cors);
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let app_state = Arc::new(AppState::init(&database_url).await);
    let app = create_app(app_state.clone());

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
