use serde::de::DeserializeOwned;
use serde::{Deserialize};
use actix_web::http::Uri;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub bind: String,
    pub secret: String,
    pub register: String,
    pub redirect: String,
}

pub fn read_file_as_unchecked<T: DeserializeOwned>(path: &str) -> T {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

pub struct AppState {
    pub secret: String,
    pub register: Uri,
    pub redirect: Uri,
}

impl From<Config> for AppState {
    fn from(src: Config) -> Self {
        use std::convert::TryFrom;

        Self {
            secret: src.secret,
            register: Uri::try_from(src.register).unwrap(),
            redirect: Uri::try_from(src.redirect).unwrap(),
        }
    }
}