// #[macro_use]
// extern crate clap;
#[macro_use]
extern crate clap;
extern crate derive_more;

use uuid::Uuid;
pub mod app;
pub mod controller;
pub mod dto;
pub mod view;

#[derive(Debug, Clone)]
pub struct Secret(String);

impl Secret {
    //pub fn new() -> Secret { Secret(Uuid::new_v4().to_string()) }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&str> for Secret {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl Default for Secret {
    fn default() -> Self {
        Secret(Uuid::new_v4().to_string())
    }
}
