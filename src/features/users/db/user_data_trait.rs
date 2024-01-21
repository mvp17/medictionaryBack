use crate::features::users::db::User;
use crate::db::Database;
use actix_web::web::Data;
use async_trait::async_trait;
use surrealdb::Error;

#[async_trait]
pub trait UserDataTrait {
    async fn get_all_users(db: &Data<Database>) -> Option<Vec<User>>;
    async fn add_user(db: &Data<Database>, new_user: User) -> Option<User>;
    async fn delete_user(db: &Data<Database>, uuid: String) -> Option<User>;
    async fn find_user_by_username(db: &Data<Database>, username: String) -> Option<User>;
}

#[async_trait]
impl UserDataTrait for Database {
    async fn get_all_users(db: &Data<Database>) -> Option<Vec<User>> {
        let result = db.client.select("user").await;
        match result {
            Ok(all_users) => Some(all_users),
            Err(_) => None,
        }
    }

    async fn add_user(db: &Data<Database>, new_user: User) -> Option<User> {
        let created_user = db.client
            .create(("user", new_user.uuid.clone()))
            .content(new_user)
            .await;
        match created_user {
            Ok(created) => created,
            Err(_) => None
        }
    }    

    async fn delete_user(db: &Data<Database>, uuid: String) -> Option<User> {
        let result: Result<Option<User>, Error> = db.client.delete(("user", &uuid)).await;
        match result {
            Ok(deleted) => deleted,
            Err(_) => None,
        }
    }

    async fn find_user_by_username(db: &Data<Database>, username: String) -> Option<User> {
        let response = db.client
        .query(format!("SELECT * FROM user WHERE username = '{}'", username))
        .await;

        match response {
            Ok(mut response) => response.take(0).unwrap(),
            Err(_) => None
        }
    }
}
