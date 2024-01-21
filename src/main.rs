use actix_web::{ web::Data, App, HttpServer, http };
use actix_cors::Cors;
mod features;
mod config;
mod db;
use crate::db::Database;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init().await.expect("error connecting to database");
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default() // allowed_origin return access-control-allow-origin: * by default
                    .allowed_origin("http://127.0.0.1:3000")
                    .allowed_origin("http://localhost:3000")
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .app_data(db_data.clone())
            .wrap(actix_web::middleware::Logger::default())
            .configure(config::app::config_services)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
