use actix_web::{App, HttpServer, web, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use actix_web::http::{Uri, StatusCode};
use serde::de::DeserializeOwned;
use actix_web::client::{Client};
use actix_web::dev::HttpResponseBuilder;
use askama::Template;
use std::ops::Fn;
use std::any::Any;
use std::iter::Map;

#[derive(Deserialize)]
pub struct InviteDTO {
    pub invitation: String,
}

#[derive(Serialize)]
pub struct AuthDTO {
    #[serde(rename = "type")]
    auth_type: String,
}

impl Default for AuthDTO {
    fn default() -> Self {
        AuthDTO { auth_type: String::from("m.login.dummy") }
    }
}

#[derive(Serialize)]
pub struct RegisterDTO {
    pub username: String,
    pub password: String,
    pub auth: AuthDTO,
}

impl RegisterDTO {
    pub fn new_default(user: String, pass: String) -> Self {
         Self {
            username: user,
            password: pass,
            auth: AuthDTO::default(),
        }
    }
}

#[derive(Deserialize)]
pub struct RegisterFormDTO {
    pub user_name: String,
    pub password: String,
    pub re_password: String,
}

fn mutate<T,F>(mut mutable: T, f: F) -> T
    where F : Fn(&mut T) -> T
{
    f(&mut mutable);
    mutable
}