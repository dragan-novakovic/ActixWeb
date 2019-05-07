use actix_web::{ HttpRequest, web, HttpResponse };

use crate::model::lot::{ Lot };

pub fn lot(req: HttpRequest) -> String {
    "GGG".to_string()
}

pub fn createLot(lot: web::Json<Lot>) -> HttpResponse {
    println!("model: {:?}", &lot);
    HttpResponse::Ok().json(lot.0)
}