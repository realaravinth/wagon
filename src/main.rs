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

use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::io;

use actix_web::{
    client::{Client, Connector},
    error::ErrorBadRequest,
    middleware::{Compress, Logger},
    web::{self, BytesMut},
    App, Error, HttpResponse, HttpServer,
};
use database::pool::get_connection_pool;
use futures::StreamExt;
use openssl::ssl::{SslConnector, SslMethod};
use std::env;
use validator::Validate;
use validator_derive::Validate;

mod database;

#[derive(Debug, Validate, Deserialize, Serialize)]
struct Email {
    #[validate(email)]
    email_id: String,
}

#[derive(Debug, Deserialize)]
struct SMTP2goResponse {
    pub request_id: String,
    pub data: SMTP2goData,
}

#[derive(Debug, Deserialize)]
struct SMTP2goData {
    pub succeeded: i32,
    pub failed: i32,
    failures: HashMap<String, String>,
    pub email_id: String,
}

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
}

async fn send_verification(data: Email, client: &Client) -> Result<(), Error> {
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

async fn get_subscriber(
    some_data: web::Json<Email>,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    send_verification(some_data.into_inner(), &client).await?;
    Ok(HttpResponse::Ok().finish())
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
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
            .service(web::resource("/join").route(web::post().to(get_subscriber)))
    })
    .bind(endpoint)?
    .run()
    .await
}
