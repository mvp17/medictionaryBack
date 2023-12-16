use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Reminder {
  pub _id: String,
  pub medicine: String,
  pub message: String,
  pub notification_time: String
}
