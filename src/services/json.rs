use ntex::web::{self, HttpResponse};
#[path = "../models/ping_type.rs"]
mod my_obj;
/// json_test handler
#[utoipa::path(
    get,
    path = "/json_test",
    responses(
      (status = 200, description = "info", body = MyObj),
    ),
  )]
#[web::get("/json_test")]
async fn json_test() -> HttpResponse {
    let t = &my_obj::MyObj {
        first_name: "Kamal".to_owned(),
        last_name: "Jamali".to_owned(),
        id: 112,
    };

    HttpResponse::Ok().json(&t) // <- send response
}

/// json_test handler
#[utoipa::path(
    post,
    path = "/json_test_post",
    request_body = MyObj,
    responses(
      (status = 201, description = "Todo created", body = MyObj),
    ),
)]
#[web::post("/json_test_post")]
async fn json_test_post(item: web::types::Json<my_obj::MyObj>) -> HttpResponse {
    println!("model: {:?}", &item.first_name);
    HttpResponse::Ok().json(&item.0) // <- send response
}

pub fn ntex_config(cfg: &mut web::ServiceConfig) {
    cfg.service(json_test);
    cfg.service(json_test_post);
}
