use actix_web::http::Uri;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use clap::{App, Arg, ArgMatches};
use uuid::Uuid;
use crate::Secret;
use std::str::FromStr;

pub const CONFIG: (&'static str, &str, &str) = ("CONFIG", "config", "c");
pub const PORT: (&'static str, &str, &str, &str) = ("PORT", "port",  "p", "7676");
pub const IP: (&'static str, &str, &str) = ("IP", "ip", "127.0.0.1");
/// The correct query name of the query string
pub const SECRET_KEY: (&'static str, &str, &str, &str) = ("SECRET_KEY", "secret-key", "-k", "invitation");
/// SECRET has a default value that is loaded from a non-static lifetime
pub const SECRET: (&'static str, &str) = ("SECRET", "secret");
/// path the user must navigate to in order to create an acc
pub const REGISTER: (&'static str, &str, &str) = ("URI_PATH", "uri-path", "register");
pub const REDIRECT: (&'static str, &str, &str) = ("REDIRECT", "redirect", "/");

pub fn init_cli<'a, 'b>(secret: &'a Secret) -> App<'a, 'b> {
    App::new("u32io Synapse Register Page")
        .version("0.0.1")
        .author("James M. <jamesjmeyer210@gmail.com>")
        .arg(Arg::with_name(CONFIG.0)
            .short(CONFIG.2)
            .long(CONFIG.1)
            .value_name("FILE")
            .help("Sets the config file")
            .takes_value(true))
        .arg(Arg::with_name(IP.0)
            .long(IP.1)
            .default_value(IP.2)
            .takes_value(true))
        .arg(Arg::with_name(PORT.0)
            .short(PORT.2)
            .long(PORT.1)
            .default_value(PORT.3)
            .takes_value(true))
        .arg(Arg::with_name(SECRET.0)
            .long(SECRET.1)
            .default_value(secret.0.as_str())
            .takes_value(true))
        .arg(Arg::with_name(REDIRECT.0)
            .long(REDIRECT.1)
            .default_value(REDIRECT.2)
            .takes_value(true))
        .arg(Arg::with_name(REGISTER.0)
            .long(REGISTER.1)
            .default_value(REGISTER.2).takes_value(true))
        .arg(Arg::with_name(SECRET_KEY.0)
            .long(SECRET_KEY.1)
            .short(SECRET_KEY.2)
            .default_value(SECRET_KEY.3))
}

#[derive(Debug, Clone)]
pub struct Config {
    pub ip: String,
    pub port: String,
    pub secret_key: String,
    pub secret: Uuid,
    pub register: String,
    pub redirect: Uri,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ip: IP.2.to_string(),
            port: PORT.3.to_string(),
            secret_key: SECRET_KEY.3.to_string(),
            secret: Uuid::new_v4(),
            register: REGISTER.2.to_string(),
            redirect: Uri::from_str(REDIRECT.2).unwrap()
        }
    }
}

impl Config {
    pub fn opts<F>(mut self, f: F) -> Self
        where
            F: Fn(&mut Self) -> (),
    {
        f(&mut self);
        self
    }
}

pub fn read_file_as_unchecked<T: DeserializeOwned>(path: &str) -> T {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

pub struct AppState {
    pub(crate) conf: Config,
}

impl AppState {
    pub fn new(conf: Config) -> Self {
        Self {
            conf,
        }
    }
}