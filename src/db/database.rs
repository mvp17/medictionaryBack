use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

use crate::models::Alarm;

#[derive(Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;
        client.signin(Root {
            username: "root",
            password: "root"
        })
        .await?;
        client.use_ns("surreal").use_db("medic").await.unwrap();
        Ok(Database {
            client,
            name_space: String::from("surreal"),
            db_name: String::from("medic")
        })
    }

    pub async fn get_all_alarms(&self) -> Option<Vec<Alarm>> {
        let result = self.client.select("alarm").await;
        match result {
            Ok(all_alarms) => Some(all_alarms),
            Err(_) => None,
        }
    }

    pub async fn add_alarm(&self, new_alarm: Alarm) -> Option<Alarm> {
        let created_alarm = self.client
                                                                .create(("alarm", new_alarm.uuid.clone()))
                                                                .content(new_alarm)
                                                                .await;
        match created_alarm {
            Ok(created ) => created,
            Err(_) => None
        }                                                            
    }

    pub async fn update_alarm(&self, uuid: String, updated_alarm: Alarm) -> Option<Alarm> {
        let find_alarm: Result<Option<Alarm>, Error> = self.client.select(("alarm", &uuid)).await;

        match find_alarm {
            Ok(found) => {
                match found {
                    Some(_found_alarm) => {
                        let updated_alarm: Result<Option<Alarm>,Error> = self
                            .client
                            .update(("alarm", &uuid))
                            .merge(Alarm {
                                uuid,
                                name: updated_alarm.name,
                                time_taking_pill: updated_alarm.time_taking_pill,
                                total_daily_amount: updated_alarm.total_daily_amount,
                                treatment_length: updated_alarm.treatment_length,
                                hour_per_dosage: updated_alarm.hour_per_dosage,
                                last_day_taking_pill: updated_alarm.last_day_taking_pill,
                                status: updated_alarm.status
                            })
                            .await;
                        match updated_alarm {
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
