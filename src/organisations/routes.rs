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
use super::handlers::*;
use actix_web::web;

#[cfg(not(tarpaulin_include))]
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/")
            .route("signin/", web::post().to(sign_in))
            .route("signup/", web::post().to(sign_up))
            .route("logout/", web::get().to(sign_out)),
    );
}
