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
use crate::payload::organisation::creds::Creds;
use crate::utils::filters::{beep, filter, forbidden};

pub fn create_new_organisation(creds: Creds) -> ServiceResult<Creds> {
    let normalised_username = &creds.name.to_lowercase().nfc().collect::<String>();
    filter(&normalised_username)?;
    forbidden(&normalised_username)?;

    beep(&normalised_username)?;

    let hash = create_hash(&creds.password)?;
    Ok(Creds::new(&normalised_username, &hash))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::errors::*;
    #[test]
    fn utils_create_new_organisation() {
        let creds = create_new_organisation(Creds::new("Realaravinth", "password")).unwrap();
        let profanity = create_new_organisation(Creds::new("fuck", "password"));
        let forbidden_creds = create_new_organisation(Creds::new(".htaccessasnc", "password"));

        assert_eq!(creds.name, "realaravinth");
        assert_eq!(profanity, Err(ServiceError::CharError));
        assert_eq!(forbidden_creds, Err(ServiceError::CharError));
    }
}
