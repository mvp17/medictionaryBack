use crate::modules::survey::db::Survey;
use crate::db::Database;
use actix_web::web::Data;
use async_trait::async_trait;
use surrealdb::Error;


#[async_trait]
pub trait SurveyDataTrait {
    async fn get_survey(db: &Data<Database>) -> Option<Vec<Survey>>;
    async fn add_survey(db: &Data<Database>, new_survey: Survey) -> Option<Survey>;
    async fn update_survey(db: &Data<Database>, uuid: String, updated_survey: Survey) -> Option<Survey>;
}

#[async_trait]
impl SurveyDataTrait for Database {
    async fn get_survey(db: &Data<Database>) -> Option<Vec<Survey>> {
        let result = db.client.select("survey").await;
        match result {
            Ok(all_surveys) => Some(all_surveys),
            Err(_) => None,
        }
    }
    
    async fn add_survey(db: &Data<Database>, new_survey: Survey) -> Option<Survey> {
        let created_survey = db.client
                                .create(("survey", new_survey.uuid.clone()))
                                .content(new_survey)
                                .await;
        match created_survey {
            Ok(created) => created,
            Err(_) => None
        }                                                            
    }

    async fn update_survey(db: &Data<Database>, uuid: String, updated_survey: Survey) -> Option<Survey> {
        let find_survey: Result<Option<Survey>, Error> = db.client.select(("survey", &uuid)).await;
        match find_survey {
            Ok(found) => {
                match found {
                    Some(_found_survey) => {
                        let updated_survey: Result<Option<Survey>, Error> = db
                            .client
                            .update(("survey", &uuid))
                            .merge(Survey {
                                uuid,
                                smoker: updated_survey.smoker,
                                drinker: updated_survey.drinker,
                                breakfast: updated_survey.breakfast,
                                lunch: updated_survey.lunch,
                                cold_md: updated_survey.cold_md,
                                prescribed: updated_survey.prescribed,
                                allergy: updated_survey.allergy
                            })
                            .await;
                        match updated_survey {
                            Ok(updated) => updated,
                            Err(_) => None
                        }
                    }
                    None => None
                }
            }
            Err(_) => None
        }
    }
}
