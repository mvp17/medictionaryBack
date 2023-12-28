use crate::models::User;
use crate::db::Database;
use actix_web::web::Data;
use async_trait::async_trait;
use surrealdb::Error;

#[async_trait]
pub trait UserDataTrait {
    async fn add_user(db: &Data<Database>, new_user: User) -> Option<User>;
    async fn get_all_users(db: &Data<Database>) -> Option<Vec<User>>;
    async fn delete_user(db: &Data<Database>, uuid: String) -> Option<User>;
    //async fn login(db: &Data<Database>, current_user: User) -> Option<User>;
    //async fn logout(db: &Data<Database>, uuid: String);
}

#[async_trait]
impl UserDataTrait for Database {
    async fn add_user(db: &Data<Database>, new_user: User) -> Option<User> {
        let find_user: Result<Option<User>, Error> = db.client.select(("user", new_user.username.clone())).await;

        match find_user {
            Ok(_) => None,
            Err(_) => {
                let created_user = db.client
                    .create(("user", new_user.username.clone()))
                    .content(new_user).await;
                match created_user {
                    Ok(created) => created,
                    Err(_) => None
                }
            }
        }
    }
    async fn get_all_users(db: &Data<Database>) -> Option<Vec<User>> {
        let result = db.client.select("user").await;
        match result {
            Ok(all_users) => Some(all_users),
            Err(_) => None,
        }
    }

    async fn delete_user(db: &Data<Database>, uuid: String) -> Option<User> {
        let result: Result<Option<User>, Error> = db.client.delete(("user", &uuid)).await;
        match result {
            Ok(deleted) => deleted,
            Err(_) => None,
        }
    }
/*
    async fn login(db: &Data<Database>, current_user: User) -> Option<User> {
                                                             
    }

    async fn logout(db: &Data<Database>, uuid: String) {
                                                             
    }
    */
}
