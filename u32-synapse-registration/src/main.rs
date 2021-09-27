use actix_web::client::Client;
use actix_web::{web, App, HttpServer};
use u32_synapse_registration::app::{read_file_as_unchecked, AppState, Config, init_cli, IP, PORT, SECRET, SECRET_KEY, REGISTER, REDIRECT};
use u32_synapse_registration::{controller, Secret};
use actix_web::test::config;
use actix_web::http::Uri;
use std::str::FromStr;
use uuid::Uuid;
use simple_logger::SimpleLogger;
use log::{info, trace, warn, error};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    SimpleLogger::new()
        .init()
        .unwrap();
    info!("Init SimpleLogger");

    let secret = Secret::new();
    let cli = init_cli(&secret);
    let args = cli.get_matches();

    let config = Config::default()
        .opts(|conf|{
            conf.ip = args.value_of(IP.0).unwrap().to_string();
            conf.port = args.value_of(PORT.0).unwrap().to_string();
            conf.secret_key = args.value_of(SECRET_KEY.0).unwrap().to_string();
            conf.secret = Uuid::from_str(args.value_of(SECRET.0).unwrap()).unwrap();
            conf.register = args.value_of(REGISTER.0).unwrap().to_string();
            conf.redirect = Uri::from_str(args.value_of(REDIRECT.0).unwrap()).unwrap();
        });
    info!("config = {:?}", config);
    let bind = format!("{}:{}", config.ip, config.port);

    let server = HttpServer::new(move || {
        let app_state = AppState::new(config.clone());

        App::new()
            .data(app_state)
            .data(Client::default())
            //.wrap()
            .route("/register", web::get().to(controller::get_index))
            .route("/register", web::post().to(controller::post_index))
    });
    info!("Server instantiated");

    server.bind(bind.as_str())
        .map_err(|e|{
            error!("Unable to bind to {} {}", &bind, &e);
            e
        })?
        .run()
        .await
}
