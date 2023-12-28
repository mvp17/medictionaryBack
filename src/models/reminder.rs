use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct ReminderDTO {
  #[validate(length(min = 1))]
  pub medicine: String,
  pub message: String,
  pub notification_time: String
}

#[derive(Validate, Serialize, Deserialize)]
pub struct Reminder {
  pub uuid: String,
  pub medicine: String,
  pub message: String,
  pub notification_time: String
}

impl Reminder {
  pub fn new(uuid: String, 
             medicine: String, 
             message: String, 
             notification_time: String) -> Reminder {
    Reminder {
      uuid,
      medicine,
      message,
      notification_time
    }
  }
}

#[derive(Validate, Deserialize, Serialize)]
pub struct UpdateReminderURL {
  pub uuid: String
}
