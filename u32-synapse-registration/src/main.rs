use actix_web::client::Client;
use actix_web::http::Uri;
use actix_web::{web, App, HttpServer};
use log::{error, info};
use simple_logger::SimpleLogger;
use std::str::FromStr;
use u32_synapse_registration::app::{init_cli, AppState, Config, DEFAULT_ADDRESS, IP, PORT, REDIRECT_URI, SECRET, SECRET_KEY, SYNAPSE_URI, BASE_URI, STATIC_PATH};
use u32_synapse_registration::{controller, Secret};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    SimpleLogger::new().init().unwrap();
    let secret = Secret::default();
    let cli = init_cli(&secret);
    let args = cli.get_matches();

    let config = Config::default().opts(|conf| {
        conf.ip = args.value_of(IP.0).unwrap().to_string();
        conf.port = args.value_of(PORT.0).unwrap().to_string();
        conf.secret_key = args.value_of(SECRET_KEY.0).unwrap().to_string();
        conf.secret = args.value_of(SECRET.0).map(|x| Secret::from(x)).unwrap();
        conf.base_uri = args.value_of(BASE_URI.0).unwrap().to_string();
        conf.redirect = args
            .value_of(REDIRECT_URI.0)
            .map(|x| Uri::from_str(x).unwrap_or(Uri::from_static(DEFAULT_ADDRESS)))
            .unwrap();
        conf.synapse = args
            .value_of(SYNAPSE_URI.0)
            .map(|x| Uri::from_str(x).unwrap_or(Uri::from_static(DEFAULT_ADDRESS)))
            .unwrap();
        conf.static_path = args.value_of(STATIC_PATH.0).unwrap().to_string();
    });
    info!("config = {:?}", config);
    let bind = format!("{}:{}", config.ip, config.port);

    let server = HttpServer::new(move || {
        let app_state = AppState::new(config.clone());

        App::new()
            .data(app_state)
            .data(Client::default())
            .route(config.base_uri.as_str(), web::get().to(controller::get_index))
            .route(config.base_uri.as_str(), web::post().to(controller::post_index))
            .service(actix_files::Files::new("/static", config.static_path.as_str())
                .show_files_listing())
    });
    info!("Server instantiated");

    server
        .max_connections(2)
        .max_connection_rate(2)
        .workers(4)
        .bind(bind.as_str())
        .map_err(|e| {
            error!("Unable to bind to {} {}", &bind, &e);
            e
        })?
        .run()
        .await
}
