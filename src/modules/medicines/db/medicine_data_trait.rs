use crate::modules::medicines::db::Medicine;
use crate::db::Database;
use actix_web::web::Data;
use async_trait::async_trait;
use surrealdb::Error;

#[async_trait]
pub trait MedicineDataTrait {
    async fn get_all_medicines(db: &Data<Database>) -> Option<Vec<Medicine>>;
    async fn get_medicine_by_id(db: &Data<Database>, uuid: String) -> Option<Medicine>;
    async fn add_medicine(db: &Data<Database>, new_medicine: Medicine) -> Option<Medicine>;
    async fn update_medicine(db: &Data<Database>, uuid: String, updated_medicine: Medicine) -> Option<Medicine>;
    async fn delete_medicine(db: &Data<Database>, uuid: String) -> Option<Medicine>;
}

#[async_trait]
impl MedicineDataTrait for Database {
    async fn get_all_medicines(db: &Data<Database>) -> Option<Vec<Medicine>> {
        let result = db.client.select("medicine").await;
        match result {
            Ok(all_medicines) => Some(all_medicines),
            Err(_) => None,
        }
    }

    async fn get_medicine_by_id(db: &Data<Database>, uuid: String) -> Option<Medicine> {
        let result = db.client.select(("medicine", &uuid)).await;
        match result {
            Ok(medicine) => medicine,
            Err(_) => None,
        }
    }

    async fn add_medicine(db: &Data<Database>, new_medicine: Medicine) -> Option<Medicine> {
        let created_medicine = db.client
                                .create(("medicine", new_medicine.uuid.clone()))
                                .content(new_medicine)
                                .await;
        match created_medicine {
            Ok(created) => created,
            Err(_) => None
        }                                                            
    } 

    async fn update_medicine(db: &Data<Database>, uuid: String, updated_medicine: Medicine) -> Option<Medicine> {
        let find_medicine: Result<Option<Medicine>, Error> = db.client.select(("medicine", &uuid)).await;
        match find_medicine {
            Ok(found) => {
                match found {
                    Some(_found_medicine) => {
                        let updated_medicine: Result<Option<Medicine>, Error> = db
                            .client
                            .update(("medicine", &uuid))
                            .merge(Medicine {
                                uuid,
                                name: updated_medicine.name,
                                description: updated_medicine.description,
                                side_effects: updated_medicine.side_effects,
                                total_daily_dosage: updated_medicine.total_daily_dosage,
                                directions_of_use: updated_medicine.directions_of_use
                            })
                            .await;
                        match updated_medicine {
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

    async fn delete_medicine(db: &Data<Database>, uuid: String) -> Option<Medicine> {
        let result: Result<Option<Medicine>, Error> = db.client.delete(("medicine", &uuid)).await;
        match result {
            Ok(deleted) => deleted,
            Err(_) => None,
        }
    }
}
