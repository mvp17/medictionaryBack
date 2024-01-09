use actix_web:: {
    http:: { header::ContentType, StatusCode },
    HttpResponse, ResponseError
};

use derive_more::Display;
#[derive(Debug, Display)]
pub enum ReminderError {
    NoRemindersFound,
    ReminderCreationFailure,
    NoSuchReminderFound,
    WrongPassword
}

impl ResponseError for ReminderError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ReminderError::NoRemindersFound => StatusCode::NOT_FOUND,
            ReminderError::ReminderCreationFailure => StatusCode::INTERNAL_SERVER_ERROR,
            ReminderError::NoSuchReminderFound => StatusCode::NOT_FOUND,
            ReminderError::WrongPassword => StatusCode::FORBIDDEN,
        }
    }
}
