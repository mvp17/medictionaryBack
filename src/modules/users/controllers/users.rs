use actix_web::HttpRequest;
use actix_web::web::Path;
use actix_web::{ web::Data, web::Json };
use crate::modules::users::error::UserError;
use crate::modules::users::db::{ DeleteUserURL, 
                                 SignUpRequestDTO, 
                                 User, 
                                 SignUpResponseDTO, 
                                 SignInRequestDTO, 
                                 SignInResponseDTO};
use validator::Validate;
use crate::modules::users::db::{ user_data_trait::UserDataTrait, Database };
use crate::modules::users::controllers::jwt::{sign_jwt, validate_request};


pub async fn signup(user: Json<SignUpRequestDTO>, db: Data<Database>) -> Result<Json<SignUpResponseDTO>, UserError> {
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
                    let response = SignUpResponseDTO { username: created.username, 
                                                                      email: created.email };
                    Ok(Json(response))
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

pub async fn signin(user_dto: Json<SignInRequestDTO>, db: Data<Database>) -> Result<Json<SignInResponseDTO>, UserError> {
    let user: Option<User> = Database::find_user_by_username(&db, user_dto.username.clone()).await;
    return if let Some(user) = user {
        if user.verify_password(&user_dto.password) {
            let token = sign_jwt(user_dto.username.clone());
            let response = SignInResponseDTO { token };
            Ok(Json(response))
        }
        else {
            Err(UserError::WrongPassword)
        }
    }
    else {
        Err(UserError::UserNotExist)
    }
}
