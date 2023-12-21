use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct MyObj {
    pub first_name: String,
    pub last_name: String,
    pub id: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct PingObject {
    pub message: String,
    pub date_time: String,
    pub ip: Option<String>,
    pub url: String,
}