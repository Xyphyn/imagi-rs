use actix_cors::Cors;
use actix_web::{App, HttpServer};
use handler::config;
use serde::Serialize;

pub mod handler;
pub mod model;
pub mod response;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    println!("Starting server...");

    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new().wrap(cors).configure(config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
