use actix_web::{App, HttpServer, web, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use actix_web::http::{Uri, StatusCode};
use serde::de::DeserializeOwned;
use actix_web::client::{Client};
use actix_web::dev::HttpResponseBuilder;
use askama::Template;
use std::ops::Fn;
use u32_synapse_registration::{view::RegisterView, controller};
use u32_synapse_registration::dto::{InviteDTO, RegisterFormDTO, RegisterDTO, AuthDTO};
use u32_synapse_registration::app::{read_file_as_unchecked, Config, AppState};

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
            .route("/", web::get().to(controller::get_index))
            .route("/", web::post().to(controller::post_index))
    })
    .bind(bind.as_str())?
    .run()
    .await
}