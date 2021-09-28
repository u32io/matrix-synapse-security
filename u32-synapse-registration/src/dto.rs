use serde::{Deserialize, Serialize};

/// Represents a query string that is sent to the controller
#[derive(Deserialize)]
pub struct InviteDTO {
    pub invitation: String,
}
/// The auth object required by synapse
#[derive(Serialize)]
pub struct AuthDTO {
    #[serde(rename = "type")]
    auth_type: String,
}
/// Initializes a default synapse value
impl Default for AuthDTO {
    fn default() -> Self {
        AuthDTO {
            auth_type: String::from("m.login.dummy"),
        }
    }
}
/// The DTO that is forwarded to synapse when this application attempts to register and account.
#[derive(Serialize)]
pub struct RegisterDTO {
    pub username: String,
    pub password: String,
    pub auth: AuthDTO,
}

impl RegisterDTO {
    /// Defaults auth, accepts a username and password
    pub fn new_default(user: String, pass: String) -> Self {
        Self {
            username: user,
            password: pass,
            auth: AuthDTO::default(),
        }
    }
}
/// The form submitted from `RegisterView`
#[derive(Deserialize)]
pub struct RegisterFormDTO {
    pub user_name: String,
    pub password: String,
    pub re_password: String,
}