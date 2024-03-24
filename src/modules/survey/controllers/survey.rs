use actix_web::HttpRequest;
use actix_web::{ web::Data, web::Json };
use crate::modules::survey::db::survey_data_trait::SurveyDataTrait;
use crate::modules::survey::error::SurveyError;
use crate::modules::survey::db::{ Survey, Database, SurveyDTO};
use validator::Validate;
use crate::modules::users::controllers::jwt::validate_request;


pub async fn register_survey(survey: Json<SurveyDTO>, 
                             req: HttpRequest, 
                             db: Data<Database>) -> Result<Json<Survey>, SurveyError> {
    match validate_request(req, &db.clone()).await {
        Ok(_) => {     
            let is_valid = survey.validate();
            match is_valid {
                Ok(_) => {
                    let smoker = survey.smoker.clone();
                    let drinker = survey.drinker.clone(); 
                    let breakfast = survey.breakfast.clone();
                    let lunch = survey.lunch.clone();
                    let cold_md = survey.cold_md.clone();
                    let prescribed = survey.prescribed.clone();
                    let allergy = survey.allergy.clone();

                    let mut buffer = uuid::Uuid::encode_buffer();
                    let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);
                    let new_survey = Database::add_survey(&db, Survey::new(
                        String::from(new_uuid),
                        smoker,
                        drinker,
                        breakfast,
                        lunch,
                        cold_md,
                        prescribed,
                        allergy
                    )).await;

                    match new_survey {
                        Some(created) => {
                            Ok(Json(created))
                        },
                        None => Err(SurveyError::SurveyCreationFailure)
                    }
                }
                Err(_) => Err(SurveyError::SurveyCreationFailure)
            }
        }
        Err(_) => Err(SurveyError::WrongPassword)
    }
}
