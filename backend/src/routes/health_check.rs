use axum::{Router, routing::get};

use crate::handlers::health_check::health_checker_handler;

pub fn get_routes() -> Router {
    Router::new().route("/", get(health_checker_handler))
}
