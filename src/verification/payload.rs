// Wagon is an independent mailing list manager
// Copyright (C) 2020  Aravinth Manivannan <realaravinth@batsense.net>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use validator::Validate;
use validator_derive::Validate;

use crate::errors::*;

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct Email {
    #[validate(email)]
    email_id: String,
}

impl Email {
    pub fn new(email_id: &str) -> ServiceResult<Self> {
        let email = Email {
            email_id: email_id.to_owned(),
        };
        email.validate()?;
        Ok(email)
    }
}

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
