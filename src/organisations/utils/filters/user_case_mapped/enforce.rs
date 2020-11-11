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
use crate::errors::{ServiceError, ServiceResult};
use crate::RE_USERNAME_CASE_MAPPED;

pub fn filter(target: &str) -> ServiceResult<()> {
    if RE_USERNAME_CASE_MAPPED.is_match(target) {
        Ok(())
    } else {
        Err(ServiceError::UsernameError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_usercase_mapped() {
        let legal = "\u{0065}";
        let illegal = "\u{0000}";
        assert_eq!(filter(legal), Ok(()));
        assert_eq!(filter(illegal), Err(ServiceError::UsernameError));
    }
}
