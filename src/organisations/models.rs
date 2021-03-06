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
use tokio_pg_mapper_derive::PostgresMapper;

use super::payload::RegisterCreds;

#[pg_mapper(table = "organisations")]
#[derive(Debug, PostgresMapper, PartialEq, Clone, Serialize, Deserialize)]
pub struct Organisation {
    pub organisation_username: String,
    pub email: String,
    pub password: String,
    pub organisation_name: Option<String>,
    pub description: Option<String>,
    pub email_verified: Option<bool>,
}

// set email_verified only when organisation_name, description and email is verified
impl From<RegisterCreds> for Organisation {
    fn from(creds: RegisterCreds) -> Self {
        Organisation {
            email: creds.email_id,
            organisation_username: creds.username,
            password: creds.password,
            organisation_name: None,
            description: None,
            email_verified: None,
        }
    }
}

impl Organisation {
    pub fn set_description(&mut self, description: &str) {
        self.description = Some(description.to_owned());
    }

    pub fn set_organisation_name(&mut self, name: &str) {
        self.organisation_name = Some(name.to_owned());
    }

    pub fn set_email_verified(&mut self) {
        self.email_verified = Some(true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn models_organisations() {
        let description = "Free software alternatives to google services";
        let organisation_name = "Shuttlecraft";
        let creds = RegisterCreds {
            email_id: "root@shuttlecraft.io".to_owned(),
            password: "password".to_owned(),
            username: "Shuttlecraft".to_owned(),
        };
        let registered_creds = creds.clone();

        let mut org: Organisation = creds.into();

        assert_eq!(
            org.email, registered_creds.email_id,
            "registered_creds -> org email"
        );
        assert_eq!(
            org.organisation_username, registered_creds.username,
            "registered_creds -> org username"
        );
        assert_eq!(
            org.password, registered_creds.password,
            "registered_creds -> org password"
        );
        assert_eq!(
            org.email, registered_creds.email_id,
            "registered_creds -> org email"
        );

        org.set_description(&description);
        assert_eq!(
            org.clone().description.unwrap(),
            description,
            "org set description"
        );

        org.set_organisation_name(&organisation_name);
        assert_eq!(
            org.clone().organisation_name.unwrap(),
            organisation_name,
            "org set organisation_name"
        );

        org.set_email_verified();
        assert_eq!(org.email_verified.unwrap(), true, "org set email verified");
    }
}
