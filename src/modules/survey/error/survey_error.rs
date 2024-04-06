use actix_web:: {
    http:: { header::ContentType, StatusCode },
    HttpResponse, ResponseError
};

use derive_more::Display;
#[derive(Debug, Display)]
pub enum SurveyError {
    SurveyCreationFailure,
    AlreadyRegistered,
    NoSuchSurveyFound,
    WrongPassword
}

impl ResponseError for SurveyError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            SurveyError::SurveyCreationFailure => StatusCode::INTERNAL_SERVER_ERROR,
            SurveyError::NoSuchSurveyFound => StatusCode::NOT_FOUND,
            SurveyError::AlreadyRegistered => StatusCode::CONFLICT,
            SurveyError::WrongPassword => StatusCode::FORBIDDEN,
        }
    }
}
