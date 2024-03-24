use crate::modules::survey::db::Survey;
use crate::db::Database;
use actix_web::web::Data;
use async_trait::async_trait;


#[async_trait]
pub trait SurveyDataTrait {
    async fn add_survey(db: &Data<Database>, new_survey: Survey) -> Option<Survey>;
}

#[async_trait]
impl SurveyDataTrait for Database {
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
}
