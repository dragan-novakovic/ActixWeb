use actix_web::{web, Error, HttpResponse};
use diesel::prelude::*;
use futures::Future;

use crate::model::user::User;
use crate::share::db::Pool;

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

// /// Diesel query
fn query_login(auth_data: AuthData, pool: web::Data<Pool>) -> Result<User, ()> {
    use crate::schema::users::dsl::*;
    let conn: &PgConnection = &pool.get().unwrap();

    let mut items = users
        .filter(email.eq(&auth_data.email))
        .limit(1)
        .load::<User>(conn)
        .unwrap();

    if let Some(user) = items.pop() {
        if user.password == auth_data.password {
            return Ok(user);
        }
    }
    Err(())
}

fn query_list(pool: web::Data<Pool>) -> Result<Vec<User>, ()> {
    use crate::schema::users::dsl::*;
    let conn: &PgConnection = &pool.get().unwrap();

    let items = users.load::<User>(conn).unwrap();
    Ok(items)
}

pub fn login_user(
    user: web::Json<AuthData>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || query_login(user.into_inner(), pool)).then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

pub fn get_user(pool: web::Data<Pool>) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || query_list(pool)).then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}
