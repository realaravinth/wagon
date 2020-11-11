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
#![warn(rust_2018_idioms, elided_lifetimes_in_paths)]
use pretty_env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use actix_web::{
    client::{Client, Connector},
    middleware::{Compress, Logger},
    web, App, HttpServer,
    error::InternalError,
    http::StatusCode,
};

use actix_http::KeepAlive;

use openssl::ssl::{SslConnector, SslMethod};
use regex::Regex;

use std::env;
use std::io;

mod database;
mod errors;
mod organisations;
mod verification;
mod subscribers;
mod settings;

use crate::organisations::{BLACKLIST, PROFAINITY, USERNAME_CASE_MAPPED};
use crate::settings::Settings;

lazy_static! {
   pub static ref RE_BLACKLIST: Regex =
        Regex::new(BLACKLIST).expect("couldn't setup blacklist list filter");
    pub static ref RE_PROFAINITY: Regex =
        Regex::new(PROFAINITY).expect("coudln't setup profainity filter");
    pub static ref RE_USERNAME_CASE_MAPPED: Regex = Regex::new(USERNAME_CASE_MAPPED)
        .expect("coudln't setup username case mapped filter");
    pub static ref SETTINGS: Settings = Settings::new().expect("couldn't load settings");
}

#[actix_web::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");

    pretty_env_logger::init();
//    static_init();

    //    let database_connection_pool = get_connection_pool(&DATABASE_URL);

    let endpoint = format!("{}:{}", &SETTINGS.server.ip, &SETTINGS.server.port);
    println!("Starting server at: {:?}", endpoint);

    HttpServer::new(move || {
        App::new()
            //           .data(database_connection_pool.clone())
            .app_data(
                web::JsonConfig::default().error_handler(|err, _| {
                    debug!("JSON deserialization error: {:?}", &err);
                    InternalError::new(err, StatusCode::BAD_REQUEST).into()
                }),
            )

            .wrap(Compress::default())
            .wrap(Logger::default())
            .data(
                Client::builder()
                    .connector(
                        Connector::new()
                            .ssl(
                                SslConnector::builder(SslMethod::tls()).unwrap().build(),
                            )
                            .finish(),
                    )
                    .finish(),
            )
            .configure(organisations::routes)
            .configure(verification::routes)
    })
//    .keep_alive(KeepAlive::Os)
    .client_timeout(0)
    .backlog(1204)
    .bind(endpoint)?
    .run()
    .await
}

#[cfg(not(tarpaulin_include))]
fn static_init(){
    use lazy_static::initialize;

    info!("Initializing statics...");
    initialize(&RE_USERNAME_CASE_MAPPED);
    initialize(&RE_PROFAINITY);
    initialize(&RE_BLACKLIST);

    info!("Initialized statics");
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    fn set_vars() {
//        env::set_var("WAGON_SMTP_API_KEY", "testing");
//        env::set_var("DATABASE_URL", "testing");
//        env::set_var("WAGON_PG_POOL_SIZE", "20");
//        env::set_var("PORT", "20");
//        env::set_var("WAGON_RD_URL", "testing");
//    }
//
//    #[test]
//    fn test_env_vars() {
//        set_vars();
//        assert_eq!(*WAGON_RD_URL, "testing");
//        assert_eq!(*WAGON_SMTP_API_KEY, "testing");
//        assert_eq!(*DATABASE_URL, "testing");
//        assert_eq!(*PORT, 20);
//        assert_eq!(*WAGON_PG_POOL_SIZE, 20);
//    }
//}
