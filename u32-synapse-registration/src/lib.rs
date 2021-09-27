// #[macro_use]
// extern crate clap;
#[macro_use]
extern crate clap;

use uuid::Uuid;

pub mod app;
pub mod controller;
pub mod dto;
pub mod view;

pub struct Secret(String);

impl Secret {
    pub fn new() -> Secret {
        Secret(Uuid::new_v4().to_string())
    }
}