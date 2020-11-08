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

use actix_identity::Identity;
use actix_web::{client::Client, web, web::Json, Error, HttpResponse, Responder};

use crate::errors::*;
use crate::payload::email::Email;
use crate::utils::send_email::send_verification;

pub async fn get_subscriber(
    some_data: web::Json<Email>,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    send_verification(some_data.into_inner(), &client).await?;
    Ok(HttpResponse::Ok().finish())
}
