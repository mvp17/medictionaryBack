use actix_web::HttpRequest;
use actix_web::web::Path;
use actix_web::{ web::Data, web::Json };
use crate::{error::UserError, models::UserDTO};
use crate::models::{User, DeleteUserURL};
use validator::Validate;
use crate::db::{ user_data_trait::UserDataTrait, Database };
use crate::controllers::jwt::{sign_jwt, validate_request};


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

pub async fn find_all_users(req: HttpRequest, db: Data<Database>) -> Result<Json<Vec<User>>, UserError> {
    match validate_request(req, &db.clone()).await {
        Ok(_) => {
            let users = Database::get_all_users(&db).await;
            match users {
                Some(found_users) => Ok(Json(found_users)),
                None => Err(UserError::NoUsersFound)
            }
        }
        Err(_) => Err(UserError::WrongPassword),
    }
}

pub async fn delete_user(delete_user_url: Path<DeleteUserURL>, req: HttpRequest, db: Data<Database>) -> Result<Json<User>, UserError> {
    match validate_request(req, &db.clone()).await {
        Ok(_) => {
            let uuid = delete_user_url.into_inner().uuid;
            let result = Database::delete_user(&db, uuid).await;

            match result {
                Some(result) => Ok(Json(result)),
                None => Err(UserError::NoSuchUserFound)
            }
        }
        Err(_) => Err(UserError::WrongPassword),
    }
}

pub async fn signin(user_dto: Json<UserDTO>, db: Data<Database>) -> Result<String, UserError> {
    let user: Option<User> = Database::find_user_by_username(&db, user_dto.username.clone()).await;
    return if let Some(user) = user {
        if user.verify_password(&user_dto.password) {
            let token = sign_jwt(user_dto.username.clone());
            Ok(token)
        }
        else {
            Err(UserError::WrongPassword)
        }
    }
    else {
        Err(UserError::UserNotExist)
    }
}
