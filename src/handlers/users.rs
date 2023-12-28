use actix_web::web::Path;
use actix_web::{ web::Data, web::Json };
use crate::{error::UserError, models::UserDTO};
use crate::models::{User, DeleteUserURL};
use validator::Validate;
use crate::db::{ user_data_trait::UserDataTrait, Database };


pub async fn signup(user: Json<UserDTO>, db: Data<Database>) -> Result<Json<User>, UserError> {
    let is_valid = user.validate();
    match is_valid {
        Ok(_) => {
            let username = user.username.clone();
            let email = user.email.clone();
            let password = user.password.clone();

            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);
            let new_user = Database::add_user(&db, User::new(
                String::from(new_uuid), 
                username, 
                email, 
                password
            )).await;

            match new_user {
                Some(created) => {
                    Ok(Json(created))
                },
                None => Err(UserError::UserCreationFailure)
            }
        }
        Err(_) => Err(UserError::UserCreationFailure)
    }
}

pub async fn find_all_users(db: Data<Database>) -> Result<Json<Vec<User>>, UserError> {
    let users = Database::get_all_users(&db).await;
    match users {
        Some(found_users) => Ok(Json(found_users)),
        None => Err(UserError::NoUsersFound)
    }
}

pub async fn delete_user(delete_user_url: Path<DeleteUserURL>, db: Data<Database>) -> Result<Json<User>, UserError> {
    let uuid = delete_user_url.into_inner().uuid;
    let result = Database::delete_user(&db, uuid).await;

    match result {
        Some(result) => Ok(Json(result)),
        None => Err(UserError::NoSuchUserFound)
    }
}

//pub async fn signin(user: Json<UserDTO>, db: Data<Database>) -> Result<>
