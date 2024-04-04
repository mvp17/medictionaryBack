use actix_web::HttpRequest;
use actix_web::{ web::Path, web::Data, web::Json };
use crate::modules::medicines::error::MedicineError;
use crate::modules::medicines::db::{ MedicineDTO, MedicineUrlUuid, Medicine };
use crate::modules::medicines::db::{ medicine_data_trait::MedicineDataTrait, Database };
use validator::Validate;
use crate::modules::users::controllers::jwt::validate_request;


pub async fn find_all_medicines(req: HttpRequest, 
                                db: Data<Database>) -> Result<Json<Vec<Medicine>>, MedicineError> {
    match validate_request(req, &db.clone()).await {
        Ok(_) => {
            let medicines = Database::get_all_medicines(&db).await;
            match medicines {
                Some(found_medicines) => Ok(Json(found_medicines)),
                None => Err(MedicineError::NoMedicinesFound)
            }
        }
        Err(_) => Err(MedicineError::WrongPassword),
    }
}

pub async fn find_medicine_by_id(medicine_url_uuid: Path<MedicineUrlUuid>, 
                                 req: HttpRequest, 
                                 db: Data<Database>) -> Result<Json<Medicine>, MedicineError> {
    match validate_request(req, &db.clone()).await {
        Ok(_) => {
            let uuid = medicine_url_uuid.into_inner().uuid;
            let medicine = Database::get_medicine_by_id(&db, uuid).await;
            match medicine {
                Some(found_medicine) => Ok(Json(found_medicine)),
                None => Err(MedicineError::NoMedicinesFound)
            }
        }
        Err(_) => Err(MedicineError::WrongPassword),
    }
}

pub async fn insert_medicine(medicine: Json<MedicineDTO>, 
                             req: HttpRequest, 
                             db: Data<Database>) -> Result<Json<Medicine>, MedicineError> {
    match validate_request(req, &db.clone()).await {
        Ok(_) => {     
            let is_valid = medicine.validate();
            match is_valid {
                Ok(_) => {
                    let name = medicine.name.clone();
                    let description = medicine.description.clone(); 
                    let side_effects = medicine.side_effects.clone();
                    let total_daily_dosage = medicine.total_daily_dosage.clone();
                    let directions_of_use = medicine.directions_of_use.clone();

                    let mut buffer = uuid::Uuid::encode_buffer();
                    let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);
                    let new_medicine = Database::add_medicine(&db, Medicine::new(
                        String::from(new_uuid),
                        name,
                        description,
                        side_effects,
                        total_daily_dosage,
                        directions_of_use
                    )).await;

                    match new_medicine {
                        Some(created) => {
                            Ok(Json(created))
                        },
                        None => Err(MedicineError::MedicineCreationFailure)
                    }
                }
                Err(_) => Err(MedicineError::MedicineCreationFailure)
            }
        }
        Err(_) => Err(MedicineError::WrongPassword),
    }
}

pub async fn update_medicine(medicine_url_uuid: Path<MedicineUrlUuid>, 
                             req: HttpRequest, db: Data<Database>, 
                             updated_medicine_request: Json<MedicineDTO>) -> Result<Json<Medicine>, MedicineError> {
    match validate_request(req, &db.clone()).await {
        Ok(_) => {
            let uuid = medicine_url_uuid.into_inner().uuid;
            let name = updated_medicine_request.name.clone();
            let description = updated_medicine_request.description.clone(); 
            let side_effects = updated_medicine_request.side_effects.clone(); 
            let total_daily_dosage = updated_medicine_request.total_daily_dosage.clone();
            let directions_of_use = updated_medicine_request.directions_of_use.clone();

            let updated_medicine = Medicine::new(
                                        String::from(uuid.clone()),
                                        name,
                                        description,
                                        side_effects,
                                        total_daily_dosage,
                                        directions_of_use
                                        );
            let update_result = Database::update_medicine(&db, uuid, updated_medicine).await;
            match update_result {
                Some(updated_medicine) => Ok(Json(updated_medicine)),
                None => Err(MedicineError::NoSuchMedicineFound)
            }
        }
        Err(_) => Err(MedicineError::WrongPassword),
    }
}

pub async fn delete_medicine(medicine_url_uuid: Path<MedicineUrlUuid>, 
                             req: HttpRequest, db: Data<Database>) -> Result<Json<Medicine>, MedicineError> {
    match validate_request(req, &db.clone()).await {
        Ok(_) => {
            let uuid = medicine_url_uuid.into_inner().uuid;
            let result = Database::delete_medicine(&db, uuid).await;

            match result {
                Some(result) => Ok(Json(result)),
                None => Err(MedicineError::NoSuchMedicineFound)
            }
        }
        Err(_) => Err(MedicineError::WrongPassword),
    }
}
