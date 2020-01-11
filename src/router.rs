use actix_web::{web, HttpResponse};

use crate::api::battle::battle;
use crate::api::factories::{
    add_player_factories, get_factories, get_player_factories, upgrade_factory, work_factory,
};
use crate::api::login::{get_player_inventory, get_user, login_user};
use crate::api::register::{create_user, delete_user};
use crate::api::time::get_time_handler;

pub fn users(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/user")
            .data(web::JsonConfig::default().limit(4096))
            .route(web::get().to(get_user))
            .route(web::post().to(create_user))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}
///ADMIN
pub fn user(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/user/{id}")
            .data(web::JsonConfig::default().limit(4096))
            .route(web::delete().to(delete_user))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

pub fn storage(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/storage")
            .data(web::JsonConfig::default().limit(4096))
            .route(web::post().to(get_player_inventory))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

pub fn login(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/login")
            .data(web::JsonConfig::default().limit(4096))
            .route(web::post().to(login_user))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

pub fn factories(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/factories")
            .route(web::get().to(get_factories))
            .route(web::post().to(get_player_factories)),
    );
}

pub fn buy_factories(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/buyFactories").route(web::post().to(add_player_factories)));
}

pub fn work_factories(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/workFactories").route(web::post().to(work_factory)));
}

pub fn upgrade_factories(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/upgradefactory").route(web::post().to(upgrade_factory)));
}

pub fn battle_controller(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/battle").route(web::post().to(battle)));
}

pub fn get_time(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/time").route(web::get().to(get_time_handler)));
}
