use actix_web::{web, Error, HttpResponse};
use diesel::prelude::*;
use futures::Future;
use uuid;

use crate::model::{factory::Factory, player::PlayerFactories};
use crate::share::db::Pool;

fn query_get_factories(pool: web::Data<Pool>) -> Result<Vec<Factory>, diesel::result::Error> {
    use crate::schema::factories::dsl::*;
    let conn: &PgConnection = &pool.get().unwrap();

    let items = factories.load::<Factory>(conn).unwrap();
    Ok(items)
}

pub fn get_factories(pool: web::Data<Pool>) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || query_get_factories(pool)).then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

fn query_get_player_factories(
    user: web::Json<UserId>,
    pool: web::Data<Pool>,
) -> Result<Vec<PlayerFactories>, diesel::result::Error> {
    use crate::schema::player_factories::dsl::*;
    let conn: &PgConnection = &pool.get().unwrap();

    let items = player_factories
        .filter(user_id.eq(&user.id))
        .load::<PlayerFactories>(conn)?;

    Ok(items)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserId {
    pub id: uuid::Uuid,
}

pub fn get_player_factories(
    user: web::Json<UserId>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || query_get_player_factories(user, pool)).then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerPayload {
    pub user_id: uuid::Uuid,
    pub factory_id: uuid::Uuid,
}

fn query_add_player_factories(
    payload: web::Json<PlayerPayload>,
    pool: web::Data<Pool>,
) -> Result<PlayerFactories, diesel::result::Error> {
    use crate::schema::player_factories::dsl::*;
    let conn: &PgConnection = &pool.get().unwrap();

    let item = player_factories
        .filter(user_id.eq(&payload.user_id))
        .filter(factory_id.eq(&payload.factory_id))
        .get_result::<PlayerFactories>(conn)
        .optional()?;

    dbg!(&item);

    match item {
        Some(data) => {
            let new_amount = data.amount + 1;

            let updated = diesel::update(player_factories)
                .filter(user_id.eq(&payload.user_id))
                .filter(factory_id.eq(&payload.factory_id))
                .set(amount.eq(new_amount))
                .get_result(conn)?;

            Ok(updated)
        }
        None => {
            let new_factories = PlayerFactories {
                user_id: payload.user_id,
                factory_id: payload.factory_id,
                amount: 1,
            };

            diesel::insert_into(player_factories)
                .values(&new_factories)
                .execute(conn)?;

            Ok(new_factories)
        }
    }
}

pub fn add_player_factories(
    player_data: web::Json<PlayerPayload>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || query_add_player_factories(player_data, pool)).then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}
