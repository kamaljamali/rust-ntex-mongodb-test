use crate::{
    db_mongodb::{connect_to_db, get_all_students2},
    models::students_model::StudentModel,
};
use chrono::Local;
use futures::{TryFutureExt, TryStreamExt};
use ntex::web::{self, HttpRequest, HttpResponse};

#[path = "../models/ping_type.rs"]
mod ping_type;

/// ping handler
#[utoipa::path(
    get,
    path = "/ping",
    responses(
        (status = 200, description = "Ping Pong", body = PingObject),
    ),
)]
#[web::get("/ping")]
pub async fn ping(req: HttpRequest) -> HttpResponse {
    let connection = connect_to_db().await;
    if let Some(client) = connection.ok() {
        let result = get_all_students2(client).await;

        if let Ok(students) = result {
            let students: Vec<StudentModel> = students
                .try_collect()
                .await
                .expect("Error collecting students");
            println!("{:#?}", students);
        }
        // if let Ok(mut students) = result {
        //     println!("{:#?}",students.try_next());
        //     while let Ok(Some(doc)) = students.try_next().await {
        //         println!("{:0}", "Ressssssssssssssssssssssssssssssss");
        //         println!("{:?}", doc);
        //     }
        // }
    }

    let mut t = String::from("No Ip");

    if let Some(val) = req.peer_addr() {
        println!("Address {:?}", val.ip());
        t = val.ip().to_string();
    };

    let t = &ping_type::PingObject {
        message: "Wellcome to my project rust ntex".to_owned(),
        date_time: Local::now().to_string(),
        ip: Some(t),
        url: req.uri().to_string(),
    };

    HttpResponse::Ok().json(&t) // <- send response
}

pub fn ntex_config(cfg: &mut web::ServiceConfig) {
    cfg.service(ping);
}
