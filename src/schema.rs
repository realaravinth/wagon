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

table! {
    lists (name) {
        name -> Varchar,
        description -> Varchar,
        organisation_name -> Nullable<Varchar>,
    }
}

table! {
    organisations (organisation_username) {
        organisation_username -> Varchar,
        organisation_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        description -> Varchar,
    }
}

table! {
    subscribers (email) {
        email -> Varchar,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(lists, organisations, subscribers,);
