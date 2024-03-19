use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct AlarmDTO {
  #[validate(length(min = 1))]
  pub name: String,
  pub time_taking_pill: String,
  pub total_daily_amount: u32,
  pub treatment_length: u32,
  pub hour_per_dosage: u32,
  pub last_day_taking_pill: u32,
  pub status: String
}

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct Alarm {
  pub uuid: String,
  pub name: String,
  pub time_taking_pill: String,
  pub total_daily_amount: u32,
  pub treatment_length: u32,
  pub hour_per_dosage: u32,
  pub last_day_taking_pill: u32,
  pub status: String
}

impl Alarm {
  pub fn new(uuid: String, 
             name: String, 
             time_taking_pill: String, 
             total_daily_amount: u32, 
             treatment_length: u32, 
             hour_per_dosage: u32, 
             last_day_taking_pill: u32, 
             status: String) -> Alarm {
    Alarm {
      uuid,
      name,
      time_taking_pill,
      total_daily_amount,
      treatment_length,
      hour_per_dosage,
      last_day_taking_pill,
      status
    }
  }
}

#[derive(Validate, Deserialize, Serialize)]
pub struct AlarmUrlUuid {
  pub uuid: String
}
