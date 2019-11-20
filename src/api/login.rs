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
#[derive(Debug, Serialize, Deserialize)]
struct UserWithData {
    pub id: uuid::Uuid,
    pub email: String,
    pub username: String,
    pub password: String,
    pub energy: i32,
    pub gold: i32,
    pub exp: i32,
}

impl From<(uuid::Uuid, String, String, String, i32, i32, i32)> for UserWithData {
    fn from(tup: (uuid::Uuid, String, String, String, i32, i32, i32)) -> UserWithData {
        UserWithData {
            id: tup.0,
            email: tup.1,
            username: tup.2,
            password: tup.3,
            energy: tup.4,
            gold: tup.5,
            exp: tup.6,
        }
    }
}

impl UserWithData {
    fn remove_pass(mut self) -> Self {
        self.password = "".to_owned();
        self
    }
}

// /// Diesel query
fn query_login(auth_data: AuthData, pool: web::Data<Pool>) -> Result<UserWithData, ()> {
    use crate::schema::players_data::dsl::{energy, exp, gold, players_data};
    use crate::schema::users::dsl::{email, id, password, username, users};
    let conn: &PgConnection = &pool.get().unwrap();

    let item: UserWithData = users
        .inner_join(players_data)
        .select((id, email, username, password, energy, gold, exp))
        .filter(email.eq(&auth_data.email))
        .get_result::<(uuid::Uuid, String, String, String, i32, i32, i32)>(conn)
        .unwrap()
        .into();

    match item.password == auth_data.password {
        true => Ok(item.remove_pass()),
        false => Err(()),
    }
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
