use sqlx::{Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
}

pub async fn connect(database_url: &str) -> AppState {}
