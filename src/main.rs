use actix_web::{ web::Path, web::Data, get, post, patch, App, HttpServer, web::Json };
use uuid;
mod models;
mod db;
mod error;
use error::{ AlarmError, MedicineError, ReminderError };
use crate::models::{ Reminder, ReminderRequest, UpdateReminderURL };
use crate::models::{ MedicineRequest, UpdateMedicineURL, Medicine };
use crate::models::{ Alarm, UpdateAlarmURL, AlarmRequest };
use validator::Validate;
use crate::db::{ alarm_data_trait::AlarmDataTrait, 
                 medicine_data_trait::MedicineDataTrait,
                 reminder_data_trait::ReminderDataTrait,
                 Database
                };

#[get("/api/alarms")]
async fn get_alarms(db: Data<Database>) -> Result<Json<Vec<Alarm>>, AlarmError> {
    let alarms = Database::get_all_alarms(&db).await;
    match alarms {
        Some(found_alarms) => Ok(Json(found_alarms)),
        None => Err(AlarmError::NoAlarmsFound)
    }
}

#[post("/api/createAlarm")]
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
            let new_alarm = Database::add_alarm(&db, Alarm::new(
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

#[patch("/api/updateAlarm/{uuid}")]
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
    let update_result = Database::update_alarm(&db, uuid, updated_alarm).await;
    match update_result {
        Some(updated_alarm) => Ok(Json(updated_alarm)),
        None => Err(AlarmError::NoSuchAlarmFound)
    }
}

#[get("/api/medicines")]
async fn get_medicines(db: Data<Database>) -> Result<Json<Vec<Medicine>>, MedicineError> {
    let medicines = Database::get_all_medicines(&db).await;
    match medicines {
        Some(found_medicines) => Ok(Json(found_medicines)),
        None => Err(MedicineError::NoMedicinesFound)
    }
}

#[post("/api/createMedicine")]
async fn create_medicine(medicine: Json<MedicineRequest>, db: Data<Database>) -> Result<Json<Medicine>, MedicineError> {
    let is_valid = medicine.validate();
    match is_valid {
        Ok(_) => {
            let name = medicine.name.clone();
            let description = medicine.description.clone(); 
            let side_effects = medicine.side_effects.clone();
            let total_daily_dosage = medicine.total_daily_dosage.clone();

            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);
            let new_medicine = Database::add_medicine(&db, Medicine::new(
                String::from(new_uuid),
                name,
                description,
                side_effects,
                total_daily_dosage
            )).await;

            match new_medicine {
                Some(created) => {
                    Ok(Json(created))
                },
                None => Err(MedicineError::MedicineCreationFailure)
            }
        }
        Err(_) => Err(MedicineError::MedicineCreationFailure)
    }
}

#[patch("/api/updateMedicine/{uuid}")]
async fn update_medicine(update_medicine_url: Path<UpdateMedicineURL>, 
                      db: Data<Database>, 
                      updated_medicine_request: Json<MedicineRequest>) -> Result<Json<Medicine>, MedicineError> {
    let uuid = update_medicine_url.into_inner().uuid;

    let name = updated_medicine_request.name.clone();
    let description = updated_medicine_request.description.clone(); 
    let side_effects = updated_medicine_request.side_effects.clone(); 
    let total_daily_dosage = updated_medicine_request.total_daily_dosage.clone();

    let updated_medicine = Medicine::new(
                                String::from(uuid.clone()),
                                name,
                                description,
                                side_effects,
                                total_daily_dosage
                                );
    let update_result = Database::update_medicine(&db, uuid, updated_medicine).await;
    match update_result {
        Some(updated_medicine) => Ok(Json(updated_medicine)),
        None => Err(MedicineError::NoSuchMedicineFound)
    }
}

#[get("/api/reminders")]
async fn get_reminders(db: Data<Database>) -> Result<Json<Vec<Reminder>>, ReminderError> {
    let reminders = Database::get_all_reminders(&db).await;
    match reminders {
        Some(found_reminders) => Ok(Json(found_reminders)),
        None => Err(ReminderError::NoRemindersFound)
    }
}

#[post("/createReminder")]
async fn create_reminder(reminder: Json<ReminderRequest>, db: Data<Database>) -> Result<Json<Reminder>, ReminderError> {
    let is_valid = reminder.validate();
    match is_valid {
        Ok(_) => {
            let medicine = reminder.medicine.clone();
            let message = reminder.message.clone(); 
            let notification_time = reminder.notification_time.clone();
            
            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);
            let new_reminder = Database::add_reminder(&db, Reminder::new(
                String::from(new_uuid),
                medicine,
                message,
                notification_time
            )).await;

            match new_reminder {
                Some(created) => {
                    Ok(Json(created))
                },
                None => Err(ReminderError::ReminderCreationFailure)
            }
        }
        Err(_) => Err(ReminderError::ReminderCreationFailure)
    }
}

#[patch("/api/updateReminder/{uuid}")]
async fn update_reminder(update_reminder_url: Path<UpdateReminderURL>, 
                      db: Data<Database>, 
                      updated_reminder_request: Json<ReminderRequest>) -> Result<Json<Reminder>, ReminderError> {
    let uuid = update_reminder_url.into_inner().uuid;

    let medicine = updated_reminder_request.medicine.clone();
    let message = updated_reminder_request.message.clone(); 
    let notification_time = updated_reminder_request.notification_time.clone();

    let updated_reminder = Reminder::new(
                                String::from(uuid.clone()),
                                medicine,
                                message,
                                notification_time
                                );
    let update_result = Database::update_reminder(&db, uuid, updated_reminder).await;
    match update_result {
        Some(updated_reminder) => Ok(Json(updated_reminder)),
        None => Err(ReminderError::NoSuchReminderFound)
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
