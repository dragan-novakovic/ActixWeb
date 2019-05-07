use actix_web::{ web, HttpResponse };


use crate::api::lots::{ lot, createLot };


pub fn lots(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/lots")
        .route(web::get().to(lot))
        .route(web::post().to(createLot))
        .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
    );
}
