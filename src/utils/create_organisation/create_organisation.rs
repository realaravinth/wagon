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
    let new_creds = RegisterCreds::new(&normalised_username, &creds.email_id, &hash)?;
    Ok(new_creds)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::errors::*;
    #[test]
    fn utils_create_new_organisation() {
        let creds = create_new_organisation(
            RegisterCreds::new("Realaravinth", "sdfa@gmail.com", "password").unwrap(),
        )
        .unwrap();
        let profane_creds =
            RegisterCreds::new("fuck", "aasdfa@gmail.com", "password").unwrap();
        let profanity = create_new_organisation(profane_creds);

        let forbidden_creds =
            RegisterCreds::new(".htaccessasnc", "sdfada@gmail.com", "password").unwrap();
        let forbidden = create_new_organisation(forbidden_creds);

        assert_eq!(creds.username, "realaravinth");
        assert_eq!(profanity, Err(ServiceError::UsernameError));
        assert_eq!(forbidden, Err(ServiceError::UsernameError));
    }
}
