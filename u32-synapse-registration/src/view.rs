use askama::Template;
use std::ops::Fn;

#[derive(Template)]
#[template(path = "index.html")]
pub struct RegisterView<'view> {
    title: &'view str,
    pub pass_mismatch: bool,
    pub query_key: &'view str,
    pub query_value: &'view str,
}

impl Default for RegisterView<'_> {
    fn default() -> Self {
        Self {
            title: "Register",
            pass_mismatch: false,
            query_key: "invite",
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
