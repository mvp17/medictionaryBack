use actix_web::web;
use actix_web::{ web::Data, App, HttpServer };
mod models;
mod handlers;
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
            .service(web::resource("/api/alarms").route(web::get().to(handlers::get_alarms)))
            .service(web::resource("/api/createAlarm").route(web::post().to(handlers::create_alarm)))
            .service(web::resource("/api/updateAlarm/{uuid}").route(web::patch().to(handlers::update_alarm)))
            .service(web::resource("/api/medicines").route(web::get().to(handlers::get_medicines)))
            .service(web::resource("/api/createMedicine").route(web::post().to(handlers::create_medicine)))
            .service(web::resource("/api/updateMedicine/{uuid}").route(web::patch().to(handlers::update_medicine)))
            .service(web::resource("/api/reminders").route(web::get().to(handlers::get_reminders)))
            .service(web::resource("/api/createReminder").route(web::post().to(handlers::create_reminder)))
            .service(web::resource("/api/updateReminder/{uuid}").route(web::patch().to(handlers::update_reminder)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
