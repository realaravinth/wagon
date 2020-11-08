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
use actix_http::http::header;

use crate::errors::*;
use crate::payload::organisation::{LoginCreds, Unvalidated_RegisterCreds, RegisterCreds};
use crate::utils:: send_email::send_verification ;

pub async fn sign_up(creds: Json<Unvalidated_RegisterCreds>) -> ServiceResult<HttpResponse> {
    debug!("inside auth::signup");

    let r_creds: ServiceResult<RegisterCreds> = creds.into_inner().into();
    let creds = r_creds?;
    
    Ok(HttpResponse::Ok().set_header(header::CONNECTION, "close").finish())
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
        let creds = Unvalidated_RegisterCreds{
            password: "password".to_owned(),
            username: "Realaravinth".to_owned(),
            email_id: "batman@we.net".to_owned()
        };

        let resp = sign_up(Json(creds)).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
    #[actix_rt::test]
    async fn test_signup_email_faliure() {
        let email_err = Unvalidated_RegisterCreds {
            username: "realaravinth".to_owned(),
            email_id: "sdfada".to_owned(),
            password: "password".to_owned(),
        };

        let email_err_resp = sign_up(Json(email_err)).await;
        assert_eq!(email_err_resp.unwrap_err(), ServiceError::NotAnEmail);
    }
}
