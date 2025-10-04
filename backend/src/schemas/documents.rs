use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDocument {
    pub date: DateTime<Utc>,
    pub inventory_number: String,
    pub scan_number: Option<String>,
    pub page_number: Option<String>,
    pub notes: Option<String>,
    pub archive_id: i32,
    pub institute_id: i32,
    pub place_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateDocument {
    pub date: Option<DateTime<Utc>>,
    pub inventory_number: Option<String>,
    pub scan_number: Option<String>,
    pub page_number: Option<String>,
    pub notes: Option<String>,
    pub archive_id: Option<i32>,
    pub institute_id: Option<i32>,
    pub place_id: Option<i32>,
}
