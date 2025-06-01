use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::models::ScanResult;
use crate::storage::DbPool;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "Server is up"}))
}

pub async fn ingest_results(
    pool: web::Data<DbPool>,
    payload: web::Json<Vec<ScanResult>>,
) -> impl Responder {
    let client = match pool.get().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("DB connection error: {}", e);
            return HttpResponse::InternalServerError()
                .json(json!({"error": "Failed to connect to database"}));
        }
    };

    for result in payload.into_inner() {
        if let Err(e) = client
            .execute(
                "INSERT INTO port_scans (ip, port, status) VALUES ($1, $2, $3)",
                &[&result.ip, &(result.port as i32), &result.status],
            )
            .await
        {
            eprintln!("Insert error: {}", e);
            return HttpResponse::InternalServerError()
                .json(json!({"error": "Failed to insert scan result"}));
        }
    }

    HttpResponse::Ok().json(json!({"status": "Scan results ingested"}))
}
