use crate::modules::alarms::db::Alarm;
use crate::db::Database;
use actix_web::web::Data;
use async_trait::async_trait;
use surrealdb::Error;

#[async_trait]
pub trait AlarmDataTrait {
    async fn get_all_alarms(db: &Data<Database>) -> Option<Vec<Alarm>>;
    async fn add_alarm(db: &Data<Database>, new_alarm: Alarm) -> Option<Alarm>;
    async fn update_alarm(db: &Data<Database>, uuid: String, updated_alarm: Alarm) -> Option<Alarm>;
    async fn delete_alarm(db: &Data<Database>, uuid: String) -> Option<Alarm>;
}

#[async_trait]
impl AlarmDataTrait for Database {
    async fn get_all_alarms(db: &Data<Database>) -> Option<Vec<Alarm>> {
        let result = db.client.select("alarm").await;
        match result {
            Ok(all_alarms) => Some(all_alarms),
            Err(_) => None,
        }
    }

    async fn add_alarm(db: &Data<Database>, new_alarm: Alarm) -> Option<Alarm> {
        let created_alarm = db.client
                                .create(("alarm", new_alarm.uuid.clone()))
                                .content(new_alarm)
                                .await;
        match created_alarm {
            Ok(created) => created,
            Err(_) => None
        }                                                            
    } 

    async fn update_alarm(db: &Data<Database>, uuid: String, updated_alarm: Alarm) -> Option<Alarm> {
        let find_alarm: Result<Option<Alarm>, Error> = db.client.select(("alarm", &uuid)).await;
        match find_alarm {
            Ok(found) => {
                match found {
                    Some(_found_alarm) => {
                        let updated_alarm: Result<Option<Alarm>, Error> = db
                            .client
                            .update(("alarm", &uuid))
                            .merge(Alarm {
                                uuid,
                                name: updated_alarm.name,
                                medicine_uuid: updated_alarm.medicine_uuid,
                                time_taking_pill: updated_alarm.time_taking_pill,
                                total_daily_amount: updated_alarm.total_daily_amount,
                                treatment_length: updated_alarm.treatment_length,
                                hour_per_dosage: updated_alarm.hour_per_dosage,
                                last_day_taking_pill: updated_alarm.last_day_taking_pill,
                                status: updated_alarm.status
                            })
                            .await;
                        match updated_alarm {
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

    async fn delete_alarm(db: &Data<Database>, uuid: String) -> Option<Alarm> {
        let result: Result<Option<Alarm>, Error> = db.client.delete(("alarm", &uuid)).await;
        match result {
            Ok(deleted) => deleted,
            Err(_) => None,
        }
    }
}
