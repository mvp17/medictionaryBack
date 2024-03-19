use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize)]
pub struct MedicineDTO {
  #[validate(length(min = 1))]
  pub name: String,
  pub description: String,
  pub side_effects: String,
  pub total_daily_dosage: u32,
}

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct Medicine {
  pub uuid: String,
  pub name: String,
  pub description: String,
  pub side_effects: String,
  pub total_daily_dosage: u32,
}

impl Medicine {
  pub fn new(uuid: String, 
             name: String, 
             description: String, 
             side_effects: String, 
             total_daily_dosage: u32) -> Medicine {
    Medicine {
      uuid,
      name,
      description,
      side_effects,
      total_daily_dosage
    }
  }
}

#[derive(Validate, Deserialize, Serialize)]
pub struct MedicineUrlUuid {
  pub uuid: String
}
