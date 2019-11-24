use actix_web::{web, HttpResponse};

use crate::api::factories::{add_player_factories, get_factories, get_player_factories};
use crate::api::login::{get_user, login_user};
use crate::api::register::{create_user, delete_user};
use crate::api::time::get_time_handler;

pub fn users(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/user")
            .data(web::JsonConfig::default().limit(4096))
            .route(web::get().to_async(get_user))
            .route(web::post().to_async(create_user))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

pub fn user(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/user/{id}")
            .data(web::JsonConfig::default().limit(4096))
            .route(web::delete().to_async(delete_user))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

pub fn login(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/login")
            .data(web::JsonConfig::default().limit(4096))
            .route(web::post().to_async(login_user))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

pub fn factories(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/factories")
            .route(web::get().to_async(get_factories))
            .route(web::post().to_async(get_player_factories)),
    );
}

pub fn buy_factories(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/buyFactories").route(web::post().to_async(add_player_factories)));
}

pub fn get_time(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/time").route(web::get().to(get_time_handler)));
}
