use actix_web::{ web::Path, web::Data, web::Json };
use crate::error::ReminderError;
use crate::models::{ Reminder, ReminderDTO, UpdateReminderURL };
use crate::db::{ reminder_data_trait::ReminderDataTrait, Database };
use validator::Validate;


pub async fn find_all_reminders(db: Data<Database>) -> Result<Json<Vec<Reminder>>, ReminderError> {
    let reminders = Database::get_all_reminders(&db).await;
    match reminders {
        Some(found_reminders) => Ok(Json(found_reminders)),
        None => Err(ReminderError::NoRemindersFound)
    }
}

pub async fn insert_reminder(reminder: Json<ReminderDTO>, db: Data<Database>) -> Result<Json<Reminder>, ReminderError> {
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

pub async fn update_reminder(update_reminder_url: Path<UpdateReminderURL>, 
                      db: Data<Database>, 
                      updated_reminder_request: Json<ReminderDTO>) -> Result<Json<Reminder>, ReminderError> {
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
