use crate::app::AppState;
use crate::dto::{InviteDTO, RegisterDTO, RegisterFormDTO};
use crate::view::RegisterView;
use actix_web::client::Client;
use actix_web::dev::HttpResponseBuilder;
use actix_web::http::{StatusCode, Uri};
use actix_web::{web, HttpResponse, Responder};
use askama::Template;

pub async fn get_index(
    invite: web::Query<InviteDTO>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    println!("GET /");
    let client_secret = &invite.invitation;
    let secret = &app_state.secret;

    match client_secret.eq(secret) {
        true => HttpResponse::Ok().body(
            RegisterView::default()
                .with(|v| v.query_value = client_secret)
                .render()
                .unwrap(),
        ),
        false => HttpResponse::Forbidden().finish(),
    }
}

pub async fn post_index(
    form: web::Form<RegisterFormDTO>,
    app_state: web::Data<AppState>,
    client: web::Data<Client>,
) -> impl Responder {
    use std::fs;
    println!("POST /");

    match &form.password.eq(&form.re_password) {
        true => {
            let resp = forward_req(
                &client,
                RegisterDTO::new_default(form.user_name.clone(), form.password.clone()),
                &app_state.register,
            )
            .await;

            match resp {
                StatusCode::OK => HttpResponse::SeeOther()
                    .header("Location", app_state.redirect.to_string())
                    .finish(),
                StatusCode::INTERNAL_SERVER_ERROR => HttpResponse::InternalServerError()
                    .body(fs::read_to_string("../templates/500.liquid").unwrap()),
                status => HttpResponseBuilder::new(status).finish(),
            }
        }
        false => HttpResponse::BadRequest().body(
            RegisterView::default()
                .with(|v| {
                    v.pass_mismatch = true;
                    v.query_key = &app_state.secret;
                })
                .render()
                .unwrap(),
        ),
    }
}

async fn forward_req(client: &Client, dto: RegisterDTO, uri: &Uri) -> StatusCode {
    println!(
        "Forward Request: {} {}",
        uri,
        serde_json::to_string(&dto).unwrap()
    );

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
    println!(
        "Client Response: {} {}",
        res.status(),
        String::from_utf8_lossy(body.as_ref())
    );

    res.status()
}
