use actix_web::{ web::Data, App, HttpServer };
mod models;
mod handlers;
mod config;
mod db;
mod error;
use crate::db::Database;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init().await.expect("error connecting to database");
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .configure(config::app::config_services)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
