use actix_web::{web, Error, HttpResponse};
use diesel::prelude::*;

use crate::model::{
    player::PlayerData,
    user::{NewUser, User},
};
use crate::share::db::Pool;

fn query(new_user_data: NewUser, pool: web::Data<Pool>) -> Result<User, diesel::result::Error> {
    use crate::schema::players_data::dsl::players_data;
    use crate::schema::users::dsl::{id, users};
    let conn: &PgConnection = &pool.get().unwrap();

    let new_player_data = PlayerData {
        id: uuid::Uuid::new_v4(),
        energy: 100,
        gold: 50,
        gold_acc: 0,
        exp: 0,
        last_updated: chrono::Utc::now().naive_utc(),
    };

    let new_user = User {
        id: uuid::Uuid::new_v4(),
        email: new_user_data.email,
        username: new_user_data.username,
        password: new_user_data.password,
        created_on: chrono::Utc::now().naive_utc(),
        player_data_id: new_player_data.id,
    };

    diesel::insert_into(players_data)
        .values(&new_player_data)
        .execute(conn)
        .unwrap();
    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
        .unwrap();

    let created_user = users.filter(id.eq(&new_user.id)).get_result(conn).unwrap();
    Ok(created_user)
}

pub async fn create_user(
    user: web::Json<NewUser>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || query(user.into_inner(), pool))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())
        .unwrap())
}

fn query_delete(
    user_id: uuid::Uuid,
    pool: web::Data<Pool>,
) -> Result<String, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let conn: &PgConnection = &pool.get().unwrap();

    diesel::delete(users.filter(id.eq(user_id)))
        .execute(conn)
        .expect("Error deleting User");

    Ok("Succes".to_owned())
}

pub async fn delete_user(
    id: web::Path<(uuid::Uuid)>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || query_delete(id.into_inner(), pool))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())
        .unwrap())
}
