mod api;
mod db;
mod error;
mod models;
mod operations;
mod schema;
use std::env;

use actix_web::{App, HttpServer};
use api::routes::configure_routes;

use crate::error::error::AppError;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(configure_routes))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
