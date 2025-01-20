use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}