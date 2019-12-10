use actix_web::{web, Error, HttpResponse};
use diesel::prelude::*;
use uuid;

use crate::model::{
    player::{PlayerData, PlayerInventory},
    user::User,
};
use crate::share::db::Pool;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BattlePayload {
    user_id: uuid::Uuid,
    battle_id: String,
}

// campign 1  => - 10 eng -X products  + special_loot
fn battle_query(
    payload: web::Json<BattlePayload>,
    pool: web::Data<Pool>,
) -> Result<String, diesel::result::Error> {
    use crate::schema::player_inventory::dsl::{player_inventory, special_currency, weapon_q1};
    use crate::schema::players_data::dsl::{energy, players_data};
    use crate::schema::users::dsl::users;
    let conn: &PgConnection = &pool.get().unwrap();

    let player: User = users.find(&payload.user_id).first(conn).unwrap();
    let curr_player_data: PlayerData = players_data
        .find(&player.player_data_id)
        .first(conn)
        .unwrap();
    let storage: PlayerInventory = player_inventory
        .filter(
            player_inventory
                .primary_key()
                .eq(&curr_player_data.player_inventory_id),
        )
        .first(conn)
        .unwrap();

    // check if enough energy
    if curr_player_data.energy < 10 {
        return Ok("Not enough energy".to_owned());
    }
    diesel::update(players_data)
        .filter(players_data.primary_key().eq(&player.player_data_id))
        .set(energy.eq(energy - 10))
        .execute(conn)
        .unwrap();
    // check if enough resourses
    if storage.weapon_q1 < 10 {
        return Ok("Not enough weapons".to_owned());
    }
    // give new resourses
    diesel::update(player_inventory)
        .filter(
            player_inventory
                .primary_key()
                .eq(&curr_player_data.player_inventory_id),
        )
        .set((
            special_currency.eq(special_currency + 5),
            weapon_q1.eq(weapon_q1 - 10),
        ))
        .execute(conn)
        .expect("Updating player_iventory loot and wp1");
    Ok(format!("Success"))
}

/// work at specific company => - 10 energy + products
pub async fn battle(
    player_data: web::Json<BattlePayload>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || battle_query(player_data, pool))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())
        .unwrap())
}
