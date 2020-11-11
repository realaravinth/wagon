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

use futures::StreamExt;

use super::payload::{Email, SMTP2goResponse};
use actix_web::{client::Client, error::ErrorBadRequest, web::BytesMut, Error};
use validator::Validate;

pub async fn send_verification(data: Email, client: &Client) -> Result<(), Error> {
    data.validate().map_err(ErrorBadRequest)?;
    let mut res = client
        .post("https://api.smtp2go.com/v3/email/send")
        .send_json(&data)
        .await
        .map_err(Error::from)?; // <- convert SendRequestError to an Error
    debug!("{:?}", res);

    if res.status() == 200 {
        return Ok(());
    }

    let mut body = BytesMut::new();
    while let Some(chunk) = res.next().await {
        body.extend_from_slice(&chunk?);
    }

    let body: SMTP2goResponse = serde_json::from_slice(&body).unwrap();
    Ok(())
}
