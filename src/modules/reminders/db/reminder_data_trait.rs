use crate::modules::reminders::db::Reminder;
use crate::db::Database;
use actix_web::web::Data;
use async_trait::async_trait;
use surrealdb::Error;

#[async_trait]
pub trait ReminderDataTrait {
    async fn get_all_reminders(db: &Data<Database>) -> Option<Vec<Reminder>>;
    async fn add_reminder(db: &Data<Database>, new_medicine: Reminder) -> Option<Reminder>;
    async fn update_reminder(db: &Data<Database>, uuid: String, updated_reminder: Reminder) -> Option<Reminder>;
}

#[async_trait]
impl ReminderDataTrait for Database {
    async fn get_all_reminders(db: &Data<Database>) -> Option<Vec<Reminder>> {
        let result = db.client.select("reminder").await;
        match result {
            Ok(all_reminders) => Some(all_reminders),
            Err(_) => None,
        }
    }

    async fn add_reminder(db: &Data<Database>, new_reminder: Reminder) -> Option<Reminder> {
        let created_reminder = db.client
            .create(("reminder", new_reminder.uuid.clone()))
            .content(new_reminder)
            .await;
        match created_reminder {
            Ok(created) => created,
            Err(_) => None
        }                                                            
    } 

    async fn update_reminder(db: &Data<Database>, uuid: String, updated_reminder: Reminder) -> Option<Reminder> {
        let find_reminder: Result<Option<Reminder>, Error> = db.client.select(("reminder", &uuid)).await;
        match find_reminder {
            Ok(found) => {
                match found {
                    Some(_found_reminder) => {
                        let updated_reminder: Result<Option<Reminder>, Error> = db
                            .client
                            .update(("reminder", &uuid))
                            .merge(Reminder {
                                uuid,
                                medicine: updated_reminder.medicine,
                                message: updated_reminder.message,
                                notification_time: updated_reminder.notification_time
                            })
                            .await;
                        match updated_reminder {
                            Ok(updated) => updated,
                            Err(_) => None
                        }
                    }
                    None => None
                }
            }
            Err(_) => None
        }
    }
}
