use crate::schema::{player_factories, players_data};
use chrono::prelude::*;
use uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Insertable)]
#[table_name = "players_data"]
pub struct PlayerData {
    pub id: uuid::Uuid,
    pub energy: i32,
    pub gold: i32,
    pub gold_acc: i32,
    pub exp: i32,
    pub last_updated: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Insertable)]
#[table_name = "player_factories"]
pub struct PlayerFactories {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub factory_id: uuid::Uuid,
    pub amount: i32,
}
