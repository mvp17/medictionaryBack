use serde::{Deserialize, Serialize};
use validator::Validate;
use bcrypt::{hash, verify};


#[derive(Validate, Serialize, Deserialize)]
pub struct UserDTO {
  #[validate(length(min = 1))]
  pub username: String,
  pub email: String,
  pub password: String,
}

#[derive(Validate, Debug, Serialize, Deserialize, Default)]
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
      password: User::hash_password(password)
    }
  }

  fn hash_password(password: String) -> String {
    hash(password, 10).unwrap()
  }

  pub fn verify_password(&self, password: &str) -> bool {
    match verify(password, &self.password) {
      Ok(b) => b,
      Err(_) => false,
    }
  }
}

#[derive(Validate, Deserialize, Serialize)]
pub struct DeleteUserURL {
  pub uuid: String
}
