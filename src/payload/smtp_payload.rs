use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct SMTP2goResponse {
    pub request_id: String,
    pub data: SMTP2goData,
}

#[derive(Debug, Deserialize)]
pub struct SMTP2goData {
    pub succeeded: i32,
    pub failed: i32,
    failures: HashMap<String, String>,
    pub email_id: String,
}
