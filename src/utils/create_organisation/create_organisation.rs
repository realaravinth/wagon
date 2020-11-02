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

use unicode_normalization::UnicodeNormalization;

use super::hashify::create_hash;
use crate::errors::ServiceResult;
use crate::payload::organisation::RegisterCreds;
use crate::utils::filters::{beep, filter, forbidden};

pub fn create_new_organisation(creds: RegisterCreds) -> ServiceResult<RegisterCreds> {
    let normalised_username = &creds.username.to_lowercase().nfc().collect::<String>();
    filter(&normalised_username)?;
    forbidden(&normalised_username)?;

    beep(&normalised_username)?;

    let hash = create_hash(&creds.password)?;
    let new_creds = RegisterCreds::new()
        .set_username(&normalised_username)
        .set_password(&hash)
        .set_email(&creds.email_id)?
        .build();

    Ok(new_creds)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::errors::*;
    #[test]
    fn utils_create_new_organisation() {
        let registered_creds = RegisterCreds::new()
            .set_password("password")
            .set_username("Realaravinth")
            .set_email("batman@we.net")
            .unwrap()
            .build();

        let org = create_new_organisation(registered_creds).unwrap();
        assert_eq!(org.username, "realaravinth");
    }

    #[test]
    fn utils_create_new_profane_organisation() {
        let profane_creds = RegisterCreds::new()
            .set_password("password")
            .set_username("fuck")
            .set_email("batman@we.net")
            .unwrap()
            .build();

        let profane_org = create_new_organisation(profane_creds);
        assert_eq!(profane_org, Err(ServiceError::UsernameError));
    }

    #[test]
    fn utils_create_new_forbidden_organisation() {
        let forbidden_creds = RegisterCreds::new()
            .set_password("password")
            .set_username("htaccessasnc")
            .set_email("batman@we.net")
            .unwrap()
            .build();

        let forbidden_org = create_new_organisation(forbidden_creds);

        assert_eq!(forbidden_org, Err(ServiceError::UsernameError));
    }
}
