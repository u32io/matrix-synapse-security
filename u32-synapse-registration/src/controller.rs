use crate::app::AppState;
use crate::dto::{InviteDTO, RegisterDTO, RegisterFormDTO};
use crate::view::{ErrorView, RegisterView};
use actix_web::client::Client;
use actix_web::dev::HttpResponseBuilder;
use actix_web::http::{StatusCode, Uri};
use actix_web::{web, HttpResponse, Responder};
use askama::Template;
use log::{error, info, trace, warn};


pub async fn get_index(
    invite: web::Query<InviteDTO>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let client_secret = &invite.invitation;
    let secret = &app_state.conf.secret;
    trace!("client_secret={}", client_secret);

    match client_secret.eq(secret.as_str()) {
        true => HttpResponse::Ok().content_type("text/html").body(
            RegisterView::default()
                .with(|v| v.query_value = client_secret)
                .render()
                .unwrap(),
        ),
        false => HttpResponse::Forbidden()
            .content_type("text/html")
            // TODO: this error message ought to be configurable
            .body(
                ErrorView::new(
                    StatusCode::FORBIDDEN.as_u16(),
                    "Looks like you weren't invited here".to_string(),
                )
                .render()
                .unwrap(),
            ),
    }
}

pub async fn post_index(
    form: web::Form<RegisterFormDTO>,
    app_state: web::Data<AppState>,
    client: web::Data<Client>,
) -> impl Responder {
    trace!("user_name={}", form.user_name);

    if !&form.password.eq(&form.re_password) {
        HttpResponse::BadRequest().content_type("text/html").body(
            RegisterView::default()
                .with(|v| {
                    v.pass_mismatch = true;
                    v.query_key = &app_state.conf.secret_key;
                })
                .render()
                .unwrap(),
        )
    } else {
        let result = forward_req(
            &client,
            RegisterDTO::new_default(form.user_name.clone(), form.password.clone()),
            &app_state.conf.redirect,
        )
        .await;

        match result {
            Err(err) => HttpResponseBuilder::new(StatusCode::from_u16(err.status as u16).unwrap())
                .content_type("text/html")
                .body(err.render().unwrap()),
            _ => HttpResponse::SeeOther()
                // TODO: make an extension for http response builder for redirects
                .content_type("text/html")
                .header("Location", app_state.conf.redirect.to_string())
                .finish(),
        }
    }
}

async fn forward_req<'view>(
    client: &'view Client,
    dto: RegisterDTO,
    uri: &'view Uri,
) -> Result<(), ErrorView<'view>> {
    trace!("uri={} username={}", uri, dto.username);

    let res = client.post(uri).send_json(&dto).await;
    if res.is_err() {
        error!("{:?}", res.unwrap_err());
        return Err(ErrorView::new(
            StatusCode::BAD_GATEWAY.as_u16(),
            "Unable to reach synapse".to_string(),
        ));
    }

    let mut res = res.unwrap();
    let body = res.body().await;
    if body.is_err() {
        error!("{:?}", body.unwrap_err());
        return Err(ErrorView::new(
            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            "An error occurred while decoding the response body".to_string(),
        ));
    }
    let body = body.unwrap();
    let body = String::from_utf8_lossy(body.as_ref());

    if StatusCode::BAD_REQUEST == res.status() {
        error!("status={} body={}", StatusCode::BAD_REQUEST, body);
        return Err(ErrorView::new(
            StatusCode::BAD_REQUEST.as_u16(),
            format!("Message from Synapse: {}", body),
        ));
    }

    println!("Client Response: {} {}", res.status(), body);
    if StatusCode::OK != res.status() {
        error!("status={}", res.status());
        return Err(ErrorView::new(
            StatusCode::BAD_GATEWAY.as_u16(),
            format!("Gateway responded with: {}", res.status()),
        ));
    }
    Ok(())
}
