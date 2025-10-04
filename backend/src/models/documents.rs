use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Document {
    pub id: i32,
    pub date: DateTime<Utc>,
    pub inventory_number: String,
    pub scan_number: String,
    pub page_number: String,
    pub notes: String,
    pub archive_id: i32,
    pub institute_id: i32,
    pub place_id: i32,
}
