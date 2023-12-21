use super::json;
use super::name;
use super::ping;
use super::student_service;
use std::sync::Arc;

use crate::errors::error::HttpError;
use crate::models::ping_type::{MyObj, PingObject};
use crate::models::students_model::{Fields, OtherStudent, StudentModel};
use ntex::http;
use ntex::util::Bytes;
use ntex::web;
use utoipa::OpenApi;

/// Main structure to generate OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        name::only_name,
        name::name_id,
        ping::ping,
        json::json_test,
        json::json_test_post,
        student_service::get_students_test,
        student_service::insert_student,
        student_service::update_student,
        student_service::insert_field_student,
    ),
    components(schemas(HttpError, PingObject, MyObj, StudentModel,OtherStudent, Fields))
)]
pub(crate) struct ApiDoc;

#[web::get("/{tail}*")]
async fn get_swagger(
    tail: web::types::Path<String>,
    openapi_conf: web::types::State<Arc<utoipa_swagger_ui::Config<'static>>>,
) -> Result<web::HttpResponse, HttpError> {
    if tail.as_ref() == "swagger.json" {
        let spec = ApiDoc::openapi().to_json().map_err(|err| HttpError {
            status: http::StatusCode::INTERNAL_SERVER_ERROR,
            msg: format!("Error generating OpenAPI spec: {}", err),
        })?;
        return Ok(web::HttpResponse::Ok()
            .content_type("application/json")
            .body(spec));
    }
    let conf = openapi_conf.as_ref().clone();
    match utoipa_swagger_ui::serve(&tail, conf.into()).map_err(|err| HttpError {
        msg: format!("Error serving Swagger UI: {}", err),
        status: http::StatusCode::INTERNAL_SERVER_ERROR,
    })? {
        None => Err(HttpError {
            status: http::StatusCode::NOT_FOUND,
            msg: format!("path not found: {}", tail),
        }),
        Some(file) => Ok({
            let bytes = Bytes::from(file.bytes.to_vec());
            web::HttpResponse::Ok()
                .content_type(file.content_type)
                .body(bytes)
        }),
    }
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
    let swagger_config =
        Arc::new(utoipa_swagger_ui::Config::new(["/explorer/swagger.json"]).use_base_layout());
    config.service(
        web::scope("/explorer")
            .state(swagger_config)
            .service(get_swagger),
    );
}
