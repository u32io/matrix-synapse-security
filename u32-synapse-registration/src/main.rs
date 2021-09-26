use actix_web::client::Client;
use actix_web::dev::Service;
use actix_web::middleware::errhandlers::ErrorHandlers;
use actix_web::{middleware, web, App, HttpServer};
use u32_synapse_registration::app::{read_file_as_unchecked, AppState, Config};
use u32_synapse_registration::controller;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config: Config = read_file_as_unchecked("config.json");
    let bind = config.bind.clone();

    println!("=== u32.io Synapse Registration ===");

    HttpServer::new(move || {
        let app_state = AppState::from(config.clone());

        App::new()
            .data(app_state)
            .data(Client::default())
            //.wrap()
            .route("/register", web::get().to(controller::get_index))
            .route("/register", web::post().to(controller::post_index))
    })
    .bind(bind.as_str())?
    .run()
    .await
}
