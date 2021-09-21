use actix_web::{App, HttpServer, web, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use actix_web::http::{Uri, StatusCode};
use serde::de::DeserializeOwned;
use actix_web::client::{Client};
use actix_web::dev::HttpResponseBuilder;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct RegisterView {
    title: String, //"Register",
    pass_mismatch: bool, //false,
    query_key: String, //"invitation",
    query_value: String //"ABCDE"
}

#[derive(Deserialize)]
struct Invite {
    invitation: String,
}

async fn get_index(invite: web::Query<Invite>, app_state: web::Data<AppState>) -> impl Responder {
    use std::fs;
    println!("GET /");
    let client_secret = &invite.invitation;
    let secret = &app_state.secret;

    match client_secret.eq(secret) {
        true => HttpResponse::Ok().body(fs::read_to_string("../templates/index.html").unwrap()),
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
                StatusCode::OK => HttpResponse::SeeOther()
                    .header("Location", app_state.redirect.to_string())
                    .finish(),
                StatusCode::INTERNAL_SERVER_ERROR => HttpResponse::InternalServerError()
                    .body(fs::read_to_string("../templates/500.liquid").unwrap()),
                status => HttpResponseBuilder::new(status).finish()
            }
        },
        false => HttpResponse::BadRequest().body(fs::read_to_string("../templates/pass_mismatch.liquid").unwrap())
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
            auth_type: "m.login.dummy".to_string(),
        }
    };

    println!("Forward Request: {} {}", uri, serde_json::to_string(&dto).unwrap());

    let res = client.post(uri).send_json(&dto).await;
    if res.is_err() {
        println!("Error: {:?}", res.unwrap_err());
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    let mut res = res.unwrap();
    let body = res.body().await;
    if body.is_err() {
        println!("Error: {:?}", body.unwrap_err());
        return StatusCode::INTERNAL_SERVER_ERROR;
    }
    let body = body.unwrap();
    println!("Client Response: {} {}", res.status(),  String::from_utf8_lossy(body.as_ref()));

    res.status()
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