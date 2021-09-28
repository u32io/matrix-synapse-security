use crate::app::{BASE_URI, SECRET_KEY};
use askama::Template;
use log::trace;
use std::ops::Fn;

#[allow(dead_code)]
#[derive(Debug, Template)]
#[template(path = "index.html")]
pub struct RegisterView<'view> {
    pub route: &'view str,
    pub pass_mismatch: bool,
    pub query_key: &'view str,
    pub query_value: &'view str,
}

impl Default for RegisterView<'_> {
    fn default() -> Self {
        Self {
            route: BASE_URI.2,
            pass_mismatch: false,
            query_key: SECRET_KEY.3,
            query_value: "",
        }
    }
}

impl<'view> RegisterView<'view> {
    pub fn with<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut Self) -> (),
    {
        f(&mut self);
        trace!("register view instantiated: {:?}", &self);
        self
    }
}

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorView<'view> {
    title: &'view str,
    message: String,
    pub status: u16,
}

impl<'view> ErrorView<'view> {
    pub fn new(status: u16, message: String) -> Self {
        ErrorView {
            title: stringify!(status),
            status,
            message,
        }
    }
}
