use actix_web::{web, HttpResponse};

use crate::api::lots::{create_lot, lot};
use crate::api::register::create_user;

pub fn lots(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/lots")
            .route(web::get().to(lot))
            .data(web::JsonConfig::default().limit(4096))
            .route(web::post().to_async(create_lot))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

pub fn users(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/user")
            .data(web::JsonConfig::default().limit(4096))
            .route(web::post().to_async(create_user))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}
