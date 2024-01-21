use actix_web:: {
    http:: { header::ContentType, StatusCode },
    HttpResponse, ResponseError
};

use derive_more::Display;
#[derive(Debug, Display)]
pub enum UserError {
    NoUsersFound,
    UserCreationFailure,
    NoSuchUserFound,
    WrongPassword,
    UserNotExist
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            UserError::NoUsersFound => StatusCode::NOT_FOUND,
            UserError::UserCreationFailure => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::NoSuchUserFound => StatusCode::NOT_FOUND,
            UserError::WrongPassword => StatusCode::FORBIDDEN,
            UserError::UserNotExist => StatusCode::BAD_REQUEST
        }
    }
}
