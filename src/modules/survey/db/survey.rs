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

impl Clone for Survey {
  fn clone(&self) -> Self {
    Survey {
      uuid: self.uuid.clone(),
      smoker: self.smoker.clone(),
      drinker: self.drinker.clone(),
      breakfast: self.breakfast.clone(),
      lunch: self.lunch.clone(),
      cold_md: self.cold_md.clone(),
      prescribed: self.prescribed.clone(),
      allergy: self.allergy.clone()
    }
  }
}

#[derive(Validate, Deserialize, Serialize)]
pub struct SurveyUrlUuid {
  pub uuid: String
}
