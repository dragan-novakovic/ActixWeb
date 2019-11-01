use actix_web::{web, Error, HttpResponse};
use diesel::prelude::*;
use futures::Future;

use crate::model::user::{NewUser, User};
use crate::share::db::Pool;

fn query(new_user_data: NewUser, pool: web::Data<Pool>) -> Result<User, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let conn: &PgConnection = &pool.get().unwrap();

    let new_user = User {
        id: uuid::Uuid::new_v4(),
        email: new_user_data.email,
        username: new_user_data.username,
        password: new_user_data.password,
        created_on: chrono::Local::now().naive_local(),
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    let mut lots_list = users.load::<User>(conn)?;
    Ok(lots_list.pop().unwrap())
}

pub fn create_user(
    user: web::Json<NewUser>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || query(user.into_inner(), pool)).then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}
