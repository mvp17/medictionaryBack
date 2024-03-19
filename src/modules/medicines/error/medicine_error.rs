use actix_web:: {
    http:: { header::ContentType, StatusCode },
    HttpResponse, ResponseError
};

use derive_more::Display;
#[derive(Debug, Display)]
pub enum MedicineError {
    NoMedicinesFound,
    MedicineCreationFailure,
    NoSuchMedicineFound,
    WrongPassword
}

impl ResponseError for MedicineError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            MedicineError::NoMedicinesFound => StatusCode::NOT_FOUND,
            MedicineError::MedicineCreationFailure => StatusCode::INTERNAL_SERVER_ERROR,
            MedicineError::NoSuchMedicineFound => StatusCode::NOT_FOUND,
            MedicineError::WrongPassword => StatusCode::FORBIDDEN,
        }
    }
}
