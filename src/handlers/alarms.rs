use actix_web::{ web::Path, web::Data, web::Json };
use crate::error::AlarmError;
use crate::models::{ Alarm, UpdateAlarmURL, AlarmDTO };
use validator::Validate;
use crate::db::{ alarm_data_trait::AlarmDataTrait, Database };


pub async fn find_all_alarms(db: Data<Database>) -> Result<Json<Vec<Alarm>>, AlarmError> {
    let alarms = Database::get_all_alarms(&db).await;
    match alarms {
        Some(found_alarms) => Ok(Json(found_alarms)),
        None => Err(AlarmError::NoAlarmsFound)
    }
}

pub async fn insert_alarm(alarm: Json<AlarmDTO>, db: Data<Database>) -> Result<Json<Alarm>, AlarmError> {
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

pub async fn update_alarm(update_alarm_url: Path<UpdateAlarmURL>, 
                      db: Data<Database>, 
                      updated_alarm_request: Json<AlarmDTO>) -> Result<Json<Alarm>, AlarmError> {
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
