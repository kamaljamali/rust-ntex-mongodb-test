use crate::{
    db_mongodb::*,
    models::students_model::{Fields, StudentModel},
};
use futures::TryStreamExt;
use mongodb::Collection;
use ntex::web::{self, HttpRequest, HttpResponse};

/// student handler
#[utoipa::path(
    get,
    path = "/student",
    responses(
        (status = 200, description = "student data", body = Vec<StudentModel>),
        (status = 500, description = "internal server error"),
    ),
)]
#[web::get("/student")]
pub async fn get_students_test(req: HttpRequest) -> HttpResponse {
    let connection = connect_to_db().await.expect("Err");
    let client = connection;
    let result = get_all_students2(client).await;

    let students = result.ok().unwrap();
    let students: Result<Vec<StudentModel>, mongodb::error::Error> = students.try_collect().await;
    match students {
        Ok(res) => HttpResponse::Ok().json(&res),
        Err(err) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

/// json_test handler
#[utoipa::path(
    post,
    path = "/student",
    request_body = StudentModel,
    responses(
      (status = 201, description = "Student created", body = StudentModel),
      (status = 500, description = "internal server error"),
    ),
)]
#[web::post("/student")]
async fn insert_student(item: web::types::Json<StudentModel>) -> HttpResponse {
    let connection = connect_to_db()
        .await
        .expect("Failed to connect to the database");
    match add_student(connection, item).await {
        Ok(result) => HttpResponse::Ok().json(&result),
        Err(err) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

/// json_test handler
#[utoipa::path(
    patch,
    path = "/student",
    request_body = StudentModel,
    responses(
      (status = 201, description = "Student updated", body = StudentModel),
      (status = 500, description = "internal server error"),
    ),
)]
#[web::patch("/student")]
async fn update_student(item: web::types::Json<StudentModel>) -> HttpResponse {
    let connection = connect_to_db()
        .await
        .expect("Failed to connect to the database");
    match update_student_test(connection, item).await {
        Ok(result) => HttpResponse::Ok().json(&result),
        Err(err) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

/// json_test handler
#[utoipa::path(
    patch,
    path = "/student/{name}",
    params(
      ("name" = String, Path, description = "Get name for sample"),
    ),
    request_body = Fields,
    responses(
      (status = 201, description = "Student updated", body = bool),
      (status = 500, description = "internal server error"),
    ),
)]
#[web::patch("/student/{name}")]
async fn insert_field_student(
    path: web::types::Path<String>,
    items: web::types::Json<Fields>,
) -> HttpResponse {
    let connection = connect_to_db()
        .await
        .expect("Failed to connect to the database");
    let name = &path;
    match update_student_fields(connection, items, name.to_string()).await {
        Ok(result) => HttpResponse::Ok().json(&result),
        Err(err) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

pub fn ntex_config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_students_test);
    cfg.service(insert_student);
    cfg.service(update_student);
    cfg.service(insert_field_student);
}
