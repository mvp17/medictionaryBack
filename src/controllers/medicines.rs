use actix_web::{ web::Path, web::Data, web::Json };
use crate::error::MedicineError;
use crate::models::{ MedicineDTO, UpdateMedicineURL, Medicine };
use crate::db::{ medicine_data_trait::MedicineDataTrait, Database };
use validator::Validate;


pub async fn find_all_medicines(db: Data<Database>) -> Result<Json<Vec<Medicine>>, MedicineError> {
    let medicines = Database::get_all_medicines(&db).await;
    match medicines {
        Some(found_medicines) => Ok(Json(found_medicines)),
        None => Err(MedicineError::NoMedicinesFound)
    }
}

pub async fn insert_medicine(medicine: Json<MedicineDTO>, db: Data<Database>) -> Result<Json<Medicine>, MedicineError> {
    let is_valid = medicine.validate();
    match is_valid {
        Ok(_) => {
            let name = medicine.name.clone();
            let description = medicine.description.clone(); 
            let side_effects = medicine.side_effects.clone();
            let total_daily_dosage = medicine.total_daily_dosage.clone();

            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);
            let new_medicine = Database::add_medicine(&db, Medicine::new(
                String::from(new_uuid),
                name,
                description,
                side_effects,
                total_daily_dosage
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

pub async fn update_medicine(update_medicine_url: Path<UpdateMedicineURL>, 
                      db: Data<Database>, 
                      updated_medicine_request: Json<MedicineDTO>) -> Result<Json<Medicine>, MedicineError> {
    let uuid = update_medicine_url.into_inner().uuid;

    let name = updated_medicine_request.name.clone();
    let description = updated_medicine_request.description.clone(); 
    let side_effects = updated_medicine_request.side_effects.clone(); 
    let total_daily_dosage = updated_medicine_request.total_daily_dosage.clone();

    let updated_medicine = Medicine::new(
                                String::from(uuid.clone()),
                                name,
                                description,
                                side_effects,
                                total_daily_dosage
                                );
    let update_result = Database::update_medicine(&db, uuid, updated_medicine).await;
    match update_result {
        Some(updated_medicine) => Ok(Json(updated_medicine)),
        None => Err(MedicineError::NoSuchMedicineFound)
    }
}
