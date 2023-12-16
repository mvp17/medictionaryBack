use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Medicine {
  pub _id: String,
  pub name: String,
  pub description: String,
  pub side_effects: String,
  pub total_daily_dosage: u32,
}
