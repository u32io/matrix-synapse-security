use crate::Secret;
use actix_web::http::Uri;
use clap::{App, Arg};
use lombok::{Builder};
use serde::de::DeserializeOwned;

use std::fs::File;
use std::io::BufReader;

use uuid::Uuid;

pub const REDIRECT_URI: (&'static str, &str, &str) = ("REDIRECT_URI", "redirect-uri", "r");
fn redirect_arg() -> Arg<'static, 'static> {
    Arg::with_name(REDIRECT_URI.0)
        .long(REDIRECT_URI.1)
        .short(REDIRECT_URI.2)
        .required(true)
        .takes_value(true)
}

pub const SYNAPSE_URI: (&'static str, &str) = ("SYNAPSE_URI", "synapse-uri");
fn synapse_arg() -> Arg<'static, 'static> {
    Arg::with_name(SYNAPSE_URI.0)
        .long(SYNAPSE_URI.1)
        .required(true)
        .takes_value(true)
}

pub const IP: (&'static str, &str, &str) = ("IP", "ip", "127.0.0.1");
fn ip_arg() -> Arg<'static, 'static> {
    Arg::with_name(IP.0)
        .long(IP.1)
        .default_value(IP.2)
        .takes_value(true)
}

pub const PORT: (&'static str, &str, &str, &str) = ("PORT", "port", "p", "7676");
fn port_arg() -> Arg<'static, 'static> {
    Arg::with_name(PORT.0)
        .short(PORT.2)
        .long(PORT.1)
        .default_value(PORT.3)
        .takes_value(true)
}

/// The correct query name of the query string
pub const SECRET_KEY: (&'static str, &str, &str, &str) =
    ("SECRET_KEY", "secret-key", "-k", "invitation");
fn secret_key_arg() -> Arg<'static, 'static> {
    Arg::with_name(SECRET_KEY.0)
        .long(SECRET_KEY.1)
        .short(SECRET_KEY.2)
        .default_value(SECRET_KEY.3)
}

/// SECRET has a default value that is loaded from a non-static lifetime
pub const SECRET: (&'static str, &str) = ("SECRET", "secret");

fn secret_arg<'a, 'b>(secret: &'a str) -> Arg<'a, 'b> {
    Arg::with_name(SECRET.0)
        .long(SECRET.1)
        .default_value(secret)
        .takes_value(true)
}

/// path the user must navigate to in order to create an acc
pub const BASE_URI: (&'static str, &str, &str) = ("base_uri", "uri-path", "/register");
fn base_uri_arg() -> Arg<'static, 'static> {
    Arg::with_name(BASE_URI.0)
        .long(BASE_URI.1)
        .default_value(BASE_URI.2)
        .takes_value(true)
}

pub const STATIC_PATH: (&'static str, &str, &str) = ("STATIC", "static", "/static");
fn static_path_arg() -> Arg<'static, 'static> {
    Arg::with_name(STATIC_PATH.0)
        .long(STATIC_PATH.1)
        .default_value(STATIC_PATH.2)
        .takes_value(true)
}

pub const APP_NAME: &'static str = "u32 Private Register for Synapse";
pub const APP_VERSION: &'static str = "0.0.1";
pub const APP_AUTHOR: &'static str = "James M. <jamesjmeyer210@gmail.com>";
pub const DEFAULT_ADDRESS: &'static str = "https://localhost:7676";

pub fn init_cli<'a, 'b>(secret: &'a Secret) -> App<'a, 'b> {
    App::new(APP_NAME)
        .version(APP_VERSION)
        .author(APP_AUTHOR)
        .arg(redirect_arg())
        .arg(synapse_arg())
        .arg(ip_arg())
        .arg(port_arg())
        .arg(secret_arg(&secret.0))
        .arg(base_uri_arg())
        .arg(secret_key_arg())
        .arg(static_path_arg())
}

#[derive(Debug, Clone, Builder)]
pub struct Config {
    pub ip: String,
    pub port: String,
    pub secret_key: String,
    pub secret: Secret,
    pub base_uri: String,
    pub redirect: Uri,
    pub synapse: Uri,
    pub static_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ip: IP.2.to_string(),
            port: PORT.3.to_string(),
            secret_key: SECRET_KEY.3.to_string(),
            secret: Secret(Uuid::new_v4().to_string()),
            base_uri: BASE_URI.2.to_string(),
            redirect: Uri::from_static(DEFAULT_ADDRESS),
            synapse: Uri::from_static(DEFAULT_ADDRESS),
            static_path: STATIC_PATH.2.to_string(),
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
        Self { conf }
    }
}
