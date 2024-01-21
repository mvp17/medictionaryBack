use crate::features::users::controllers::JsonMessage;
use crate::features::users::db::User;
use actix_web::web::Data;
use crate::features::users::db::{ user_data_trait::UserDataTrait, Database };
use actix_web::{HttpRequest, HttpResponse};
use jsonwebtoken::errors::Error as JwtError;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    sub: String,
    iat: usize,
    exp: usize,
}

pub async fn validate_request(req: HttpRequest, db: &Data<Database>) -> Result<String, HttpResponse> {
    let access_token = match req.headers().get("Authorization") {
        Some(token) => match token.to_str() {
            Ok(token) => token.replace("Bearer ", ""),
            Err(_) => {
                return Err(HttpResponse::InternalServerError().json(JsonMessage {
                    msg: "Error parsing header to string".to_string(),
                }));
            }
        },
        None => {
            return Err(HttpResponse::InternalServerError().json(JsonMessage {
                msg: "Authorization field not exist".to_string(),
            }));
        }
    };

    match validate_jwt(access_token) {
        Ok(sub) => {
            let user: Option<User> = Database::find_user_by_username(&db, sub).await;
            return if let Some(user) = user {
                Ok(user.username)
            }
            else {
                Err(HttpResponse::Unauthorized().json(JsonMessage {
                    msg: "User not found".to_string(),
                }))
            }
        },
        // JwtError
        Err(e) => {
            return Err(HttpResponse::Unauthorized().json(JsonMessage {
                msg: format!("{:?}", e),
            }));
        }
    };
}

pub fn sign_jwt(sub: String) -> String {
    let key = "sdfvsdgf4986sr4f1se65f1";
    let key = key.as_bytes();
    let now = chrono::Utc::now().timestamp() as usize;

    let claims = Claims {
        sub: sub.to_owned(),
        iat: now,
        // add 5min (300 sec) to now
        exp: now + 300,
    };
    let token = jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(key))
        .expect("Failed to sign token");
    token
}

fn validate_jwt(token: String) -> Result<String, JwtError> {
    let key = "sdfvsdgf4986sr4f1se65f1";
    let key = key.as_bytes();
    let validation = Validation::new(Algorithm::HS256);
    // let mut validation = Validation::new(Algorithm::HS256);
    // validation.sub = Some(sub.clone());
    match jsonwebtoken::decode::<Claims>(&token, &DecodingKey::from_secret(key), &validation) {
        Ok(data) => Ok(data.claims.sub),
        Err(err) => Err(err),
    }
}