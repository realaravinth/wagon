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
    pub fn new() -> Self {
        let registered_creds: RegisterCreds = Default::default();
        registered_creds
    }

    pub fn set_username<'a>(&'a mut self, username: &str) -> &'a mut Self {
        self.username = username.trim().to_owned();
        self
    }

    pub fn set_email<'a>(&'a mut self, email_id: &str) -> ServiceResult<&'a mut Self> {
        self.email_id = email_id.trim().to_owned();
        self.validate()?;
        Ok(self)
    }

    pub fn set_password<'a>(&'a mut self, password: &str) -> &'a mut Self {
        self.password = password.to_owned();
        self
    }

    pub fn build(&mut self) -> Self {
        self.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utils_register_builer() {
        let registered_creds = RegisterCreds::new()
            .set_password("password")
            .set_username("realaravinth")
            .set_email("batman@we.net")
            .unwrap()
            .build();

        assert_eq!(registered_creds.username, "realaravinth");
        assert_eq!(registered_creds.password, "password");
        assert_eq!(registered_creds.email_id, "batman@we.net");
    }

    #[test]
    fn utils_register_email_err() {
        let mut email_err = RegisterCreds::new()
            .set_password("password")
            .set_username("realaravinth")
            .build();
        assert_eq!(
            email_err.set_email("sdfasdf"),
            Err(ServiceError::NotAnEmail)
        );
    }
}
