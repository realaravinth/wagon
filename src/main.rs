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
#[macro_use]
extern crate diesel;

use actix_web::{
    client::{Client, Connector},
    middleware::{Compress, Logger},
    web, App, HttpServer,
};

use openssl::ssl::{SslConnector, SslMethod};
use regex::Regex;

use std::env;
use std::io;

mod database;
mod errors;
mod handlers;
mod payload;
mod routes;
mod schema;
mod utils;

use crate::utils::filters::{BLACKLIST, PROFAINITY, USERNAME_CASE_MAPPED};
use database::pool::get_connection_pool;

lazy_static! {
    pub static ref WAGON_SMTP_API_KEY: String =
        env::var("WAGON_SMTP_API_KEY").expect("Please set WAGON_SMTP_API_KEY to your SMTP API key");
    pub static ref DATABASE_URL: String =
        env::var("DATABASE_URL").expect("Please set DATABASE_URL to your postgres instance");
    pub static ref WAGON_PG_POOL_SIZE: u32 = env::var("WAGON_PG_POOL_SIZE")
        .expect("Please set WAGON_PG_POOL_SIZE to your postgres instance")
        .parse()
        .expect("Couldn't convert WAGON_PG_POOL_SIZE to integer");
    pub static ref PORT: u32 = env::var("PORT")
        .expect("Please set PORT to the port that you wish to listen to")
        .parse()
        .expect("Couldn't convert port into an integer");
    pub static ref WAGON_RD_URL: String =
        env::var("WAGON_RD_URL").expect("Please set WAGON_RD_URL to your Redis instance");
    pub static ref RE_BLACKLIST: Regex =
        Regex::new(BLACKLIST).expect("couldn't setup blacklist list filter");
    pub static ref RE_PROFAINITY: Regex =
        Regex::new(PROFAINITY).expect("coudln't setup profainity filter");
    pub static ref RE_USERNAME_CASE_MAPPED: Regex =
        Regex::new(USERNAME_CASE_MAPPED).expect("coudln't setup username case mapped filter");
}

#[actix_web::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> io::Result<()> {
    use crate::routes::routes;
    env::set_var("RUST_LOG", "actix_web=info");
    pretty_env_logger::init();

    let database_connection_pool = get_connection_pool(&DATABASE_URL);

    let endpoint = format!("0.0.0.0:{}", *PORT);
    println!("Starting server at: {:?}", endpoint);

    HttpServer::new(move || {
        App::new()
            .data(database_connection_pool.clone())
            .wrap(Compress::default())
            .wrap(Logger::default())
            .data(
                Client::builder()
                    .connector(
                        Connector::new()
                            .ssl(SslConnector::builder(SslMethod::tls()).unwrap().build())
                            .finish(),
                    )
                    .finish(),
            )
            .configure(routes)
    })
    .bind(endpoint)?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set_vars() {
        env::set_var("WAGON_SMTP_API_KEY", "testing");
        env::set_var("DATABASE_URL", "testing");
        env::set_var("WAGON_PG_POOL_SIZE", "20");
        env::set_var("PORT", "20");
        env::set_var("WAGON_RD_URL", "testing");
    }

    #[test]
    fn test_env_vars() {
        set_vars();
        assert_eq!(*WAGON_RD_URL, "testing");
        assert_eq!(*WAGON_SMTP_API_KEY, "testing");
        assert_eq!(*DATABASE_URL, "testing");
        assert_eq!(*PORT, 20);
        assert_eq!(*WAGON_PG_POOL_SIZE, 20);
    }
}
