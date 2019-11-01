use actix_web::{web, Error, HttpRequest, HttpResponse};
use diesel::prelude::*;
use futures::Future;

use crate::model::lot::{Lot, NewLot};
use crate::share::db::Pool;

fn query(lot: NewLot, pool: web::Data<Pool>) -> Result<Lot, diesel::result::Error> {
    use crate::schema::lots::dsl::*;

    let new_lot = Lot {
        id: uuid::Uuid::new_v4(),
        name: lot.name,
        description: Some(lot.description),
        price: lot.price,
    };
    let conn: &PgConnection = &pool.get().unwrap();

    diesel::insert_into(lots).values(&new_lot).execute(conn)?;

    let mut lots_list = lots.load::<Lot>(conn)?;
    Ok(lots_list.pop().unwrap())
}

pub fn lot(_req: HttpRequest) -> String {
    "GGG".to_string()
}

pub fn create_lot(
    lot: web::Json<NewLot>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || query(lot.into_inner(), pool)).then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}
