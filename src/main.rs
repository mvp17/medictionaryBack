use actix_web::{ web::Path, web::Data, get, post, patch, App, HttpServer, web::Json};
use uuid;
mod models;
mod db;
mod error;
use error::AlarmError;
use crate::db::Database;
use crate::models::{Alarm, UpdateAlarmURL, AlarmRequest};
use validator::Validate;

#[get("/alarms")]
async fn get_alarms(db: Data<Database>) -> Result<Json<Vec<Alarm>>, AlarmError> {
    let alarms = db.get_all_alarms().await;
    match alarms {
        Some(found_pizzas) => Ok(Json(found_pizzas)),
        None => Err(AlarmError::NoAlarmsFound)
    }
}

#[post("/createAlarm")]
async fn create_alarm(alarm: Json<AlarmRequest>, db: Data<Database>) -> Result<Json<Alarm>, AlarmError> {
    let is_valid = alarm.validate();
    match is_valid {
        Ok(_) => {
            let name = alarm.name.clone();
            let time_taking_pill = alarm.time_taking_pill.clone(); 
            let total_daily_amount = alarm.total_daily_amount.clone(); 
            let treatment_length = alarm.treatment_length.clone();
            let hour_per_dosage = alarm.hour_per_dosage.clone();
            let last_day_taking_pill = alarm.last_day_taking_pill.clone();
            let status = alarm.status.clone();

            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);
            let new_alarm = db.add_alarm(Alarm::new(
                String::from(new_uuid),
                name,
                time_taking_pill,
                total_daily_amount,
                treatment_length,
                hour_per_dosage,
                last_day_taking_pill,
                status
            )).await;

            match new_alarm {
                Some(created) => {
                    Ok(Json(created))
                },
                None => Err(AlarmError::AlarmCreationFailure)
            }
        }
        Err(_) => Err(AlarmError::AlarmCreationFailure)
    }
}

#[patch("/updateAlarm/{uuid}")]
async fn update_alarm(update_alarm_url: Path<UpdateAlarmURL>, 
                      db: Data<Database>, 
                      updated_alarm_request: Json<AlarmRequest>) -> Result<Json<Alarm>, AlarmError> {
    let uuid = update_alarm_url.into_inner().uuid;

    let name = updated_alarm_request.name.clone();
    let time_taking_pill = updated_alarm_request.time_taking_pill.clone(); 
    let total_daily_amount = updated_alarm_request.total_daily_amount.clone(); 
    let treatment_length = updated_alarm_request.treatment_length.clone();
    let hour_per_dosage = updated_alarm_request.hour_per_dosage.clone();
    let last_day_taking_pill = updated_alarm_request.last_day_taking_pill.clone();
    let status = updated_alarm_request.status.clone();

    let updated_alarm = Alarm::new(
                                String::from(uuid.clone()),
                                name,
                                time_taking_pill,
                                total_daily_amount,
                                treatment_length,
                                hour_per_dosage,
                                last_day_taking_pill,
                                status
                                );
    let update_result = db.update_alarm(uuid, updated_alarm).await;
    match update_result {
        Some(updated_alarm) => Ok(Json(updated_alarm)),
        None => Err(AlarmError::NoSuchAlarmFound)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init().await.expect("error connecting to database");
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_alarms)
            .service(create_alarm)
            .service(update_alarm)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
