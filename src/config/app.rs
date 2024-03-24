use actix_web::web;
use crate::modules::alarms::controllers::alarms;
use crate::modules::users::controllers::users;
use crate::modules::medicines::controllers::medicines;
use crate::modules::reminders::controllers::reminders;
use crate::modules::survey::controllers::survey;


pub fn config_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .service(
                        web::resource("/signup").route(web::post().to(users::signup)),
                    )
                    .service(
                        web::resource("/signin").route(web::post().to(users::signin)),
                    )
                    .service(
                        web::resource("/get-users").route(web::get().to(users::find_all_users)),
                    )
                    .service(
                        web::resource("/delete-user/{uuid}").route(web::delete().to(users::delete_user)),
                    )
                    /*
                    .service(
                        web::resource("/signout").route(web::post().to(users::signout)),
                    )*/
            )
            .service(
                web::scope("/alarms")
                    .service(
                        web::resource("")
                            .route(web::get().to(alarms::find_all_alarms))
                            .route(web::post().to(alarms::insert_alarm)),
                    )
                    .service(
                        web::resource("/{uuid}")
                            //.route(web::get().to(alarms::find_by_id))
                            .route(web::put().to(alarms::update_alarm))
                            .route(web::delete().to(alarms::delete_alarm)),
                    )
            )
            .service(
                web::scope("/medicines")
                    .service(
                        web::resource("")
                            .route(web::get().to(medicines::find_all_medicines))
                            .route(web::post().to(medicines::insert_medicine)),
                    )
                    .service(
                        web::resource("/{uuid}")
                            //.route(web::get().to(medicines::find_by_id))
                            .route(web::put().to(medicines::update_medicine))
                            .route(web::delete().to(medicines::delete_medicine)),
                    )
            )
            .service(
                web::scope("/reminders")
                    .service(
                        web::resource("")
                            .route(web::get().to(reminders::find_all_reminders))
                            .route(web::post().to(reminders::insert_reminder)),
                    )
                    .service(
                        web::resource("/{uuid}")
                            //.route(web::get().to(reminders::find_by_id))
                            .route(web::put().to(reminders::update_reminder))
                            .route(web::delete().to(reminders::delete_reminder)),
                    )
            )
            .service(
                web::scope("/survey")
                    .service(
                        web::resource("")
                            .route(web::post().to(survey::register_survey)),
                    )
            )
    );
}
