use db::db_mongodb;
use ntex::web::{self, middleware, App, HttpRequest};
mod db;
mod errors;
mod models;
// mod schemas;
mod services;
use dotenv::dotenv;
use std::env;

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!"
}

#[ntex::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    dotenv().ok();

    let connection = db_mongodb::connect_to_db().await;
    if let Some(client) = connection.ok(){
        db_mongodb::test_connection(client).await;        
    }

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    
    web::server(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // Register swagger endpoints
            .configure(services::openapi::ntex_config)
            // // Register todo endpoints
            .configure(services::name::ntex_config)
            .configure(services::json::ntex_config)
            .configure(services::ping::ntex_config)
            .configure(services::student_service::ntex_config)
            // // Default endpoint for unregisterd endpoints
            .default_service(web::route().to(services::default))
            .service((
                web::resource("/index.html").to(|| async { "Hello world!" }),
                web::resource("/").to(index),
            ))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
