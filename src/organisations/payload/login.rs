// Wagon is an independent mailing list manager
// Csopyright (C) 2020  Aravinth Manivannan <realaravinth@batsense.net>
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
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct LoginCreds {
    pub username: String,
    pub password: String,
}

impl LoginCreds {
    #[cfg(not(tarpaulin_include))]
    pub fn new(username: &str, password: &str) -> LoginCreds {
        LoginCreds {
            username: username.to_owned(),
            password: password.to_owned(),
        }
    }
}
