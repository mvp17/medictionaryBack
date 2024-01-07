pub mod alarms;
pub use alarms::find_all_alarms;
pub use alarms::insert_alarm;
pub use alarms::update_alarm;
pub mod medicines;
pub use medicines::find_all_medicines;
pub use medicines::insert_medicine;
pub use medicines::update_medicine;
pub mod reminders;
pub use reminders::find_all_reminders;
pub use reminders::insert_reminder;
pub use reminders::update_reminder;
pub mod users;
pub mod jwt;
pub use jwt::sign_jwt;

use serde::Serialize;

#[derive(Serialize)]
pub struct JsonMessage {
    pub msg: String,
}
