use actix_web:: {
    http:: { header::ContentType, StatusCode },
    HttpResponse, ResponseError
};

use derive_more::Display;
#[derive(Debug, Display)]
pub enum AlarmError {
    NoAlarmsFound,
    AlarmCreationFailure,
    NoSuchAlarmFound,
    WrongPassword
}

impl ResponseError for AlarmError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AlarmError::NoAlarmsFound => StatusCode::NOT_FOUND,
            AlarmError::AlarmCreationFailure => StatusCode::INTERNAL_SERVER_ERROR,
            AlarmError::NoSuchAlarmFound => StatusCode::NOT_FOUND,
            AlarmError::WrongPassword => StatusCode::FORBIDDEN,
        }
    }
}
