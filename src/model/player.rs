use crate::schema::{player_factories, players_data};
use uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Insertable)]
#[table_name = "players_data"]
pub struct PlayerData {
    pub energy: i32,
    pub gold: i32,
    pub exp: i32,
    pub user_id: uuid::Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Insertable)]
#[table_name = "player_factories"]
pub struct PlayerFactories {
    pub user_id: uuid::Uuid,
    pub factory_id: uuid::Uuid,
    pub amount: i32,
}
