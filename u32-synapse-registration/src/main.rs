use actix_web::{App, HttpServer, web, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use actix_web::http::{Uri, StatusCode};
use serde::de::DeserializeOwned;
use actix_web::client::{Client};
use actix_web::dev::HttpResponseBuilder;

#[derive(Deserialize)]
struct Invite {
    invitation: String,
}

async fn get_index(invite: web::Query<Invite>, app_state: web::Data<AppState>) -> impl Responder {
    use std::fs;
    println!("GET /");

    let secret = &app_state.secret;
    let client_secret = &invite.invitation;

    match client_secret.eq(secret) {
        true => HttpResponse::Ok().body(fs::read_to_string("static/index.html").unwrap()),
        false => HttpResponse::Forbidden().finish(),
    }
}

#[derive(Deserialize)]
struct RegisterForm {
    user_name: String,
    password: String,
    re_password: String,
}

async fn post_index(form: web::Form<RegisterForm>, app_state: web::Data<AppState>, client: web::Data<Client>) -> impl Responder {
    use std::fs;
    println!("POST /");

    match &form.password.eq(&form.re_password) {
        true => {
            match forward_req(&client,&form, &app_state.register).await {
                StatusCode::OK => HttpResponse::TemporaryRedirect()
                    .header("Location", app_state.redirect.to_string())
                    .finish(),
                StatusCode::INTERNAL_SERVER_ERROR => HttpResponse::InternalServerError()
                    .body(fs::read_to_string("static/500.html").unwrap()),
                status => HttpResponseBuilder::new(status).finish()
            }
        },
        false => HttpResponse::BadRequest().body(fs::read_to_string("static/pass_mismatch.html").unwrap())
    }
}

#[derive(Serialize)]
struct AuthDTO {
    #[serde(rename = "type")]
    auth_type: String,
}

#[derive(Serialize)]
struct RegisterDTO {
    username: String,
    password: String,
    auth: AuthDTO,
}

async fn forward_req(client: &Client, req: &RegisterForm, uri: &Uri) -> StatusCode {
    let dto = RegisterDTO {
        username: req.user_name.to_string(),
        password: req.password.to_string(),
        auth: AuthDTO {
            auth_type: "m.login.password".to_string(),
        }
    };

    match client.post(uri)
        .send_json(&dto)
        .await
        .map_err(|e|{
            println!("Error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })
        .and_then(|res| {
            println!("Response: {}", res.status());
            Ok(res.status())
        })
    {
        Ok(s) => s,
        Err(s) => s
    }
}

#[derive(Deserialize, Clone)]
struct Config {
    bind: String,
    secret: String,
    register: String,
    redirect: String,
}

pub fn read_file_as_unchecked<T: DeserializeOwned>(path: &str) -> T {
    use std::fs::File;
    use std::io::BufReader;

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

struct AppState {
    secret: String,
    register: Uri,
    redirect: Uri,
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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config: Config = read_file_as_unchecked("config.json");
    let bind = config.bind.clone();

    println!("=== u32.io Synapse Registration ===");

    HttpServer::new(move ||{
        let app_state = AppState::from(config.clone());

        App::new()
            .data(app_state)
            .data(Client::default())
            .route("/", web::get().to(get_index))
            .route("/", web::post().to(post_index))
    })
    .bind(bind.as_str())?
    .run()
    .await
}