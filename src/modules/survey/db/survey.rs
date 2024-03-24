use serde::{Serialize, Deserialize};
use validator::Validate;


#[derive(Validate, Serialize, Deserialize)]
pub struct SurveyDTO {
  pub smoker: u32,
  pub drinker: u32,
  #[validate(length(min = 1))]
  pub breakfast: String,
  #[validate(length(min = 1))]
  pub lunch: String,
  pub cold_md: String,
  pub prescribed: String,
  pub allergy: String,
}

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct Survey {
  pub uuid: String,
  pub smoker: u32,
  pub drinker: u32,
  pub breakfast: String,
  pub lunch: String,
  pub cold_md: String,
  pub prescribed: String,
  pub allergy: String,
}

impl Survey {
  pub fn new(uuid: String, 
            smoker: u32, 
            drinker: u32,
            breakfast: String, 
            lunch: String, 
            cold_md: String,
            prescribed: String,
            allergy: String) -> Survey {
    Survey {
      uuid,
      smoker,
      drinker,
      breakfast,
      lunch,
      cold_md,
      prescribed,
      allergy
    }
  }
}
