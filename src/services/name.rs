use ntex::web::{self, Error, HttpResponse};
use ntex::{channel::mpsc, util::Bytes};

/// only_name handler
#[utoipa::path(
    get,
    path = "/only_name/{name}",
    params(("name" = String, Path, description = "Get name for sample")),
    responses((status = 200, description = "name of path", body = String)),
  )]
#[web::get("/only_name/{name}")]
async fn only_name(path: web::types::Path<String>) -> HttpResponse {
    println!("Your path is: {:?}", path);
    let name = &path;
    let text = format!("Hello dear {}!", *name);

    let (tx, rx_body) = mpsc::channel();
    let _ = tx.send(Ok::<_, Error>(Bytes::from(text)));

    HttpResponse::Ok().streaming(rx_body)
}

/// name_id handler
#[utoipa::path(
    get,
    path = "/name_id/{name}/{id}",
    params(
      ("name" = String, Path, description = "Get name for sample"),
      ("id" = i32,Path,description = "id of user")
    ),
    responses(
      (status = 200, description = "name and id of path", body = [String]),
    ),
  )]
#[web::get("/name_id/{name}/{id}")]
async fn name_id(path: web::types::Path<(String, String)>) -> HttpResponse {
    println!("Your path is: {:?}", path);
    let (name, id) = &path.into_inner();
    println!("Your name is: {:?} and id {:?}", name, id);
    let text = format!("Hello dear {} with Id {}!", *name, *id);

    let (tx, rx_body) = mpsc::channel();
    let _ = tx.send(Ok::<_, Error>(Bytes::from(text)));

    HttpResponse::Ok().streaming(rx_body)
}
pub fn ntex_config(cfg: &mut web::ServiceConfig) {
    cfg.service(only_name);
    cfg.service(name_id);
}
