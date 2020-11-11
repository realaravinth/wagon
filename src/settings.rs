use std::env;

use config::{Config, Environment, File};
use serde::Deserialize;

use crate::errors::*;

#[derive(Debug, Clone, Deserialize)]
pub struct Postgres {
    hostname: String,
    port: u32,
    username: String,
    password: String,
    name: String,
    pub pool: u32,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SMTP2Go {
    pub key: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub port: u32,
    pub ip: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub postgres: Postgres,
    pub smtp2go: SMTP2Go,
    pub server: Server,
}

impl Settings {
    pub fn new() -> ServiceResult<Self> {
        let mut s = Config::new();
        s.merge(File::with_name("config/default"))
            .expect("couldn't find config/default.toml");

        let env = env::var("WAGON_RUN_MODE").unwrap_or_else(|_| "development".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))
            .expect(&format!("Couldn't find config/{}", env));

        s.merge(File::with_name("config/local").required(false))?;

        s.merge(Environment::with_prefix("WAGON").separator("_"))
            .unwrap();

        match env::var("PORT") {
            Ok(val) => {
                s.set("server.port", val).unwrap();
                println!("")
            }
            Err(e) => println!("couldn't interpret PORT: {}", e),
        }

        s.set(
            "postgres.url",
            format!(
                r"postgres://{}:{}@{}:{}/{}",
                s.get::<String>("postgres.username")
                    .expect("Couldn't access Postgres database username"),
                s.get::<String>("postgres.password")
                    .expect("Couldn't access Postgres database password"),
                s.get::<String>("postgres.hostname")
                    .expect("Couldn't access Postgres database hostname"),
                s.get::<String>("postgres.port")
                    .expect("Couldn't access Postgres database port"),
                s.get::<String>("postgres.name")
                    .expect("Couldn't access Postgres database name")
            ),
        )?;

        Ok(s.try_into().unwrap())
    }
}
