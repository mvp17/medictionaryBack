pub mod users;
pub mod jwt;

use serde::Serialize;

#[derive(Serialize)]
pub struct JsonMessage {
    pub msg: String,
}
