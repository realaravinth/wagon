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
use actix_web::{web, web::Json, Error, HttpResponse, Responder};

use crate::errors::*;
use crate::payload::organisation::{LoginCreds, RegisterCreds};
use crate::utils::{
    create_organisation::create_new_organisation, send_email::send_verification,
};

pub async fn sign_up(creds: Json<RegisterCreds>) -> ServiceResult<HttpResponse> {
    let new_creds = create_new_organisation(creds.into_inner())?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn sign_in(
    creds: web::Json<LoginCreds>,
    id: Identity,
) -> ServiceResult<impl Responder> {
    unimplemented!();
    Ok(HttpResponse::Ok().finish())
}

pub async fn sign_out(id: Identity) -> ServiceResult<impl Responder> {
    id.forget();
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body("You are successfully signed out"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http;

    #[actix_rt::test]
    async fn test_signup_success() {
        let creds = RegisterCreds::new()
            .set_password("password")
            .set_username("Realaravinth")
            .set_email("batman@we.net")
            .unwrap()
            .build();

        let resp = sign_up(Json(creds)).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
    #[actix_rt::test]
    async fn test_signup_email_faliure() {
        let email_err = RegisterCreds {
            username: "realaravinth".to_owned(),
            email_id: "sdfada".to_owned(),
            password: "password".to_owned(),
        };

        let email_err_resp = sign_up(Json(email_err)).await;
        assert_eq!(email_err_resp.unwrap_err(), ServiceError::NotAnEmail);
    }
}
