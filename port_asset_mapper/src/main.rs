use actix_web::{web, App, HttpServer};
use storage::{init_pool, DbPool};
use utilities::run_migration;

mod handlers;
mod models;
mod storage;
mod utilities;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listening_nic = "127.0.0.1";
    let listening_port = "6969";
    let address = format!("{}:{}", listening_nic, listening_port);

    println!("Initializing Database on first run");
    let pool = init_pool().await.expect("DB Pool Failed");
    run_migration(&pool).await.expect("Migration Failed");

    println!("Starting server on http://{}", address);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/health", web::get().to(handlers::health_check))
            .route("/ingest", web::post().to(handlers::ingest_results))
    })
    .bind(address)?
    .run()
    .await
}
