use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

// #[derive(Clone)] allows to wrap AppState in Arc and clone it for routes.
#[derive(Clone)]
pub struct AppState {
    pool: Pool<Postgres>,
}

impl AppState {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }

    pub async fn init(database_url: &str) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
            .unwrap_or_else(|err| {
                eprintln!("🔥 Failed to connect to the database: {:?}", err);
                std::process::exit(1);
            });

        println!("✅ Connection to the database is successful!");

        Self::new(pool)
    }
}
