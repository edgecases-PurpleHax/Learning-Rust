use actix_web::{web, App, HttpServer};

mod handlers;
mod models;
mod storage;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listening_nic = "127.0.0.1";
    let listening_port = "6969";
    let address = format!("{}:{}", listening_nic, listening_port);
    println!("Starting server on http://{}", address);
    HttpServer::new(|| App::new().route("/health", web::get().to(handlers::health_check)))
        .bind(address)?
        .run()
        .await
}
