use serde::{Deserialize, Serialize};
use validator::Validate;



#[derive(Validate, Serialize, Deserialize)]
pub struct UserDTO {
  #[validate(length(min = 1))]
  pub username: String,
  pub email: String,
  pub password: String,
}

#[derive(Validate, Debug, Serialize, Deserialize)]
pub struct User {
  pub uuid: String,
  pub username: String,
  pub email: String,
  pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
  pub username_or_email: String,
  pub password: String,
}

impl User {
  pub fn new(uuid: String, 
              username: String, 
              email: String, 
              password: String) -> User {
    User {
      uuid,
      username,
      email,
      password
    }
  }
}

#[derive(Validate, Deserialize, Serialize)]
pub struct DeleteUserURL {
  pub uuid: String
}
