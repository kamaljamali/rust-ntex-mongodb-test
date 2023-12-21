// use crate::schemas::students_sch::students;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct Fields {
    pub fields: Vec<String>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct OtherStudent {
    pub test: String,
    pub admin: bool,
    pub birthdate: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct StudentModel {
    // _id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    pub average_dip: i32,
    pub other: OtherStudent,
    pub fields: Vec<String>,
}
