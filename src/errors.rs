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

use actix_http::ResponseBuilder;
use actix_web::{
    error::{BlockingError, ResponseError},
    http::header,
    http::StatusCode,
    HttpResponse,
};
//use diesel::result::Error as DBError;
use failure::Fail;
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;
use validator::ValidationErrors;

use std::convert::From;

#[derive(Debug, PartialEq, Fail)]
pub enum ServiceError {
    #[fail(display = "some characters are not permitted")] //405j
    UsernameError,
    #[fail(display = "organisation exists")] //405
    OrganisationExists,
    #[fail(display = "invalid credentials")]
    AuthorizationRequired,
    #[fail(display = "internal error")] // 500
    InternalServerError,
    #[fail(display = "timeout")] //408
    Timeout,
    #[fail(display = "bad request")] //400
    BadRequest,
    #[fail(display = "Unable to connect to DB")]
    UnableToConnectToDb,
    #[fail(display = "The value you entered for email is not an email")] //405j
    NotAnEmail,
}

#[derive(Serialize, Deserialize)]
struct ErrorToResponse {
    error: String,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        ResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json; charset=UTF-8")
            .json(ErrorToResponse {
                error: self.to_string(),
            })
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ServiceError::UsernameError => StatusCode::METHOD_NOT_ALLOWED,
            ServiceError::OrganisationExists => StatusCode::METHOD_NOT_ALLOWED,
            ServiceError::AuthorizationRequired => StatusCode::UNAUTHORIZED,
            ServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::BadRequest => StatusCode::BAD_REQUEST,
            ServiceError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            ServiceError::UnableToConnectToDb => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::NotAnEmail => StatusCode::BAD_REQUEST,
        }
    }
}

impl From<BlockingError<ServiceError>> for ServiceError {
    fn from(_: BlockingError<ServiceError>) -> ServiceError {
        ServiceError::InternalServerError
    }
}

impl From<actix_http::Error> for ServiceError {
    fn from(_: actix_http::Error) -> ServiceError {
        ServiceError::InternalServerError
    }
}

impl From<argon2::Error> for ServiceError {
    fn from(_: argon2::Error) -> ServiceError {
        ServiceError::InternalServerError
    }
}

impl From<ValidationErrors> for ServiceError {
    fn from(_: ValidationErrors) -> ServiceError {
        ServiceError::NotAnEmail
    }
}

impl From<oneshot::error::RecvError> for ServiceError {
    fn from(_: oneshot::error::RecvError) -> ServiceError {
        ServiceError::InternalServerError
    }
}

pub type ServiceResult<V> = std::result::Result<V, crate::errors::ServiceError>;
