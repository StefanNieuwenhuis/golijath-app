use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePlace {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePlace {
    pub name: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}
