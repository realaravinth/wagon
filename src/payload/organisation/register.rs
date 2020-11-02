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

use serde::{Deserialize, Serialize};
use validator::Validate;
use validator_derive::Validate;

use crate::errors::*;

#[derive(Debug, Default, Clone, PartialEq, Validate, Deserialize, Serialize)]
pub struct RegisterCreds {
    pub username: String,
    #[validate(email)]
    pub email_id: String,
    pub password: String,
}

impl RegisterCreds {
    pub fn new(
        username: &str,
        email_id: &str,
        password: &str,
    ) -> ServiceResult<RegisterCreds> {
        let new_creds = RegisterCreds {
            username: username.to_owned(),
            email_id: email_id.into(),
            password: password.to_owned(),
        };
        new_creds.validate()?;
        Ok(new_creds)
    }

    fn set_username(&mut self, username: &str) {
        self.username = username.to_owned();
    }

    fn set_email(&mut self, email_id: &str) {
        self.email_id = email_id.to_owned();
    }

    fn set_passwordd(&mut self, password: &str) {
        self.password = password.to_owned();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utils_register_email_err() {
        let email_err = RegisterCreds::new("sdfasdfc", "sdfada", "password");
        assert_eq!(email_err, Err(ServiceError::NotAnEmail));
    }
}
