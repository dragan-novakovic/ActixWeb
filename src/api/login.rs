use actix_web::{web, Error, HttpResponse};
use chrono;
use chrono::prelude::*;
use diesel::prelude::*;

use crate::api::time::get_current_time_diff;
use crate::model::player::PlayerData;
use crate::model::user::{User, UserId};
use crate::share::db::Pool;

/// CHECK FOR DATETIME IF NOT NOW UPDATE !!!

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
    pub player_data: PlayerData,
}

impl
    From<(
        uuid::Uuid,
        String,
        String,
        String,
        uuid::Uuid,
        i32,
        i32,
        i32,
        NaiveDateTime,
        i32,
        uuid::Uuid,
        uuid::Uuid,
    )> for UserWithData
{
    fn from(
        tup: (
            uuid::Uuid,
            String,
            String,
            String,
            uuid::Uuid,
            i32,
            i32,
            i32,
            NaiveDateTime,
            i32,
            uuid::Uuid,
            uuid::Uuid,
        ),
    ) -> UserWithData {
        UserWithData {
            id: tup.0,
            email: tup.1,
            username: tup.2,
            password: tup.3,
            player_data: PlayerData {
                id: tup.4,
                energy: tup.5,
                gold: tup.6,
                exp: tup.7,
                last_updated: tup.8,
                gold_acc: tup.9,
                player_stats_id: tup.10,
                player_inventory_id: tup.11,
            },
        }
    }
}

impl UserWithData {
    fn remove_pass(mut self) -> Self {
        self.password = "".to_owned();
        self
    }

    fn update_gold(mut self) -> Self {
        self.player_data.gold = self.player_data.gold
            + get_current_time_diff(self.player_data.last_updated) * self.player_data.gold_acc;
        self
    }

    fn new_gold(&self) -> i32 {
        self.player_data.gold
            + get_current_time_diff(self.player_data.last_updated) * self.player_data.gold_acc
    }
}

// /// Diesel query
fn query_login(auth_data: AuthData, pool: web::Data<Pool>) -> Result<UserWithData, ()> {
    use crate::schema::players_data::dsl::{
        energy, exp, gold, gold_acc, last_updated, player_inventory_id, player_stats_id,
        players_data,
    };
    use crate::schema::users::dsl::{email, id, password, username, users};
    let conn: &PgConnection = &pool.get().unwrap();

    // get invetory and stats
    // check capacity

    let item: UserWithData = users
        .inner_join(players_data)
        .select((
            id,
            email,
            username,
            password,
            players_data.primary_key(),
            energy,
            exp,
            gold,
            last_updated,
            gold_acc,
            player_inventory_id,
            player_stats_id,
        ))
        .filter(email.eq(&auth_data.email))
        .get_result::<(
            uuid::Uuid,
            String,
            String,
            String,
            uuid::Uuid,
            i32,
            i32,
            i32,
            NaiveDateTime,
            i32,
            uuid::Uuid,
            uuid::Uuid,
        )>(conn)
        .expect("User With Data Error (Join)")
        .into();

    diesel::update(players_data)
        .set((
            gold.eq(item.new_gold()),
            last_updated.eq(chrono::Utc::now().naive_utc()),
        ))
        .execute(conn)
        .expect("Updating player data (Login)");

    match item.password == auth_data.password {
        true => Ok(item.update_gold().remove_pass()),
        false => Err(()),
    }
}

fn query_list(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let conn: &PgConnection = &pool.get().unwrap();

    let items = users.load::<User>(conn).expect("Loading users list");
    Ok(items)
}

pub async fn login_user(
    user: web::Json<AuthData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || query_login(user.into_inner(), pool))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())
        .expect("General login_user error?"))
}

pub async fn get_user(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || query_list(pool))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())
        .expect("Get_user general"))
}

///USER INVENTORY
fn query_user_inventory(
    _user: web::Json<UserId>,
    pool: web::Data<Pool>,
) -> Result<Vec<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let conn: &PgConnection = &pool.get().unwrap();

    let items = users.load::<User>(conn).expect("Loading users list");
    Ok(items)
}

pub async fn get_player_inventory(
    user: web::Json<UserId>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || query_user_inventory(user, pool))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())
        .expect("Get_user general"))
}
