use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateArchive {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateArchive {
    pub name: Option<String>,
}
