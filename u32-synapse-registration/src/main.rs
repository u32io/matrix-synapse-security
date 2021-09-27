use actix_web::client::Client;
use actix_web::http::uri::Scheme;
use actix_web::http::Uri;
use actix_web::test::config;
use actix_web::{web, App, HttpServer};
use log::{error, info, trace, warn};
use simple_logger::SimpleLogger;
use std::str::FromStr;
use u32_synapse_registration::app::{
    init_cli, read_file_as_unchecked, AppState, Config, DEFAULT_ADDRESS, IP, PORT, REDIRECT_URI,
    SECRET, SECRET_KEY, SYNAPSE_URI, URI_PATH,
};
use u32_synapse_registration::{controller, Secret};
use uuid::Uuid;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    SimpleLogger::new().init().unwrap();
    info!("Init SimpleLogger");

    let secret = Secret::default();
    let cli = init_cli(&secret);
    let args = cli.get_matches();

    let config = Config::default().opts(|conf| {
        info!("building config");
        conf.ip = args.value_of(IP.0).unwrap().to_string();
        conf.port = args.value_of(PORT.0).unwrap().to_string();
        conf.secret_key = args.value_of(SECRET_KEY.0).unwrap().to_string();
        conf.secret = args.value_of(SECRET.0).map(|x| Secret::from(x)).unwrap();
        info!("conf.secret={}", conf.secret.as_str());
        conf.uri_path = args.value_of(URI_PATH.0).unwrap().to_string();
        conf.redirect = args
            .value_of(REDIRECT_URI.0)
            .map(|x| Uri::from_str(x).unwrap_or(Uri::from_static(DEFAULT_ADDRESS)))
            .unwrap();
        conf.synapse = args
            .value_of(SYNAPSE_URI.0)
            .map(|x| Uri::from_str(x).unwrap_or(Uri::from_static(DEFAULT_ADDRESS)))
            .unwrap();
        info!("config built")
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

    server
        .bind(bind.as_str())
        .map_err(|e| {
            error!("Unable to bind to {} {}", &bind, &e);
            e
        })?
        .run()
        .await
}
