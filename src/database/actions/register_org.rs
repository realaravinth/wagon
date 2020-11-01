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
//
use diesel::prelude::*;

use crate::database::models::Organisations;
use crate::errors::*;

//pub fn insert_new_user(payload: RegisterRequestPayload, conn: &PgConnection) -> ServiceResult<()> {
//    use crate::schema::users::dsl::*;
//    let insertable: InsertableUser = payload.into();
//    diesel::insert_into(users)
//        .values(insertable)
//        .execute(conn)?;
//
//    Ok(())
//}
