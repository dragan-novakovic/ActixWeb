use actix_web::{ HttpRequest, web, HttpResponse };

use crate::model::lot::{ Lot };

pub fn lot(_req: HttpRequest) -> String {
    "GGG".to_string()
}

pub fn create_lot(lot: web::Json<Lot>) -> HttpResponse {
    println!("model: {:?}", &lot);
    HttpResponse::Ok().json(lot.0)
}