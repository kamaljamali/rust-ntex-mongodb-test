pub mod openapi;
pub mod name;
pub mod json;
pub mod ping;
pub mod student_service;

use ntex::web;

pub async fn default() -> web::HttpResponse {
  web::HttpResponse::NotFound().finish()
}