use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Place {
    pub id: i32,
    pub name: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}
