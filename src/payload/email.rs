use serde::{Deserialize, Serialize};
use validator::Validate;
use validator_derive::Validate;

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct Email {
    #[validate(email)]
    email_id: String,
}
