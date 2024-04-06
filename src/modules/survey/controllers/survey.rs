use actix_web::HttpRequest;
use actix_web::{ web::Data, web::Json, web::Path };
use crate::modules::survey::db::survey_data_trait::SurveyDataTrait;
use crate::modules::survey::error::SurveyError;
use crate::modules::survey::db::{ Survey, Database, SurveyDTO, SurveyUrlUuid};
use validator::Validate;
use crate::modules::users::controllers::jwt::validate_request;
use std::sync::{Arc, Mutex};


lazy_static::lazy_static! {
    static ref SURVEY_INSTANCE: Arc<Mutex<Option<Survey>>> = Arc::new(Mutex::new(None));
}

pub async fn find_survey(req: HttpRequest, db: Data<Database>) -> Result<Json<Vec<Survey>>, SurveyError> {
    match validate_request(req, &db.clone()).await {
        Ok(_) => {
            let survey = Database::get_survey(&db).await;
            match survey {
                Some(found_survey) => Ok(Json(found_survey)),
                None => Err(SurveyError::NoSuchSurveyFound)
            }
        }
        Err(_) => Err(SurveyError::WrongPassword),
    }
}

pub async fn register_survey(survey: Json<SurveyDTO>, 
                             req: HttpRequest, 
                             db: Data<Database>) -> Result<Json<Survey>, SurveyError> {
    match validate_request(req, &db.clone()).await {
        Ok(_) => {
            let mut survey_instance = SURVEY_INSTANCE.lock().unwrap();

            if survey_instance.is_some() {
                return Err(SurveyError::AlreadyRegistered);
            }

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

                    let new_survey = Survey::new(
                        String::from(new_uuid),
                        smoker,
                        drinker,
                        breakfast,
                        lunch,
                        cold_md,
                        prescribed,
                        allergy
                    );

                    *survey_instance = Some(new_survey.clone());

                    let created_survey = Database::add_survey(&db, new_survey.clone()).await;

                    match created_survey {
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

pub async fn update_survey(survey_url_uuid: Path<SurveyUrlUuid>, 
                           req: HttpRequest, db: Data<Database>, 
                           updated_survey_request: Json<SurveyDTO>) -> Result<Json<Survey>, SurveyError> {
    match validate_request(req, &db.clone()).await {
        Ok(_) => {
            let uuid = survey_url_uuid.into_inner().uuid;
            let smoker = updated_survey_request.smoker.clone();
            let drinker = updated_survey_request.drinker.clone(); 
            let breakfast = updated_survey_request.breakfast.clone(); 
            let lunch = updated_survey_request.lunch.clone();
            let cold_md = updated_survey_request.cold_md.clone();
            let prescribed = updated_survey_request.prescribed.clone();
            let allergy = updated_survey_request.allergy.clone();

            let updated_survey = Survey::new(
                String::from(uuid.clone()),
                smoker,
                drinker,
                breakfast,
                lunch,
                cold_md,
                prescribed,
                allergy
            );
            let update_result = Database::update_survey(&db, uuid, updated_survey).await;
            match update_result {
                Some(updated_survey) => Ok(Json(updated_survey)),
                None => Err(SurveyError::NoSuchSurveyFound)
            }
        }
        Err(_) => Err(SurveyError::WrongPassword),
    }
}
