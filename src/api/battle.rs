use actix_web::{web, Error, HttpResponse};
use diesel::prelude::*;
use uuid;

use crate::model::{
    factory::Factory,
    player::{PlayerData, PlayerFactories, PlayerInventory},
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
    // use crate::schema::factories::dsl::factories;
    // use crate::schema::player_factories::dsl::{amount, factory_id, player_factories, user_id};
    use crate::schema::player_inventory::dsl::{food_q1, player_inventory, weapon_q1};
    use crate::schema::players_data::dsl::{energy, gold_acc, players_data};
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

    // ?. check if has storage space

    // 1. Take from player_data -10 energy
    diesel::update(players_data)
        .filter(players_data.primary_key().eq(&player.player_data_id))
        .set(energy.eq(energy - 10))
        .execute(conn)
        .unwrap();
    // 2. add specific factory product to player inventory
    // ?check if he owns that company
    // let item = player_factories
    //     .filter(user_id.eq(&payload.user_id))
    //     .filter(factory_id.eq(&payload.factory_id))
    //     .get_result::<PlayerFactories>(conn)
    //     .optional()?;

    // diesel::update(players_data)
    //     .filter(players_data.primary_key().eq(&player.player_data_id))
    //     .set(gold_acc.eq(gold_acc + current_factory.gold_per_day))
    //     .execute(conn)
    //     .unwrap();

    //new_factories
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
