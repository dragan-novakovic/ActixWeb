use crate::schema::{player_factories, player_inventory, player_stats, players_data};
use chrono::prelude::*;
use uuid;

// MODELS PROPERTIES MUST BE IN SAME ORDER AS COLUMNS IN TABLE

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Insertable)]
#[table_name = "players_data"]
pub struct PlayerData {
    pub energy: i32,
    pub gold: i32,
    pub exp: i32,
    pub id: uuid::Uuid,
    pub last_updated: NaiveDateTime,
    pub gold_acc: i32,
    pub player_stats_id: uuid::Uuid,
    pub player_inventory_id: uuid::Uuid,
}

impl Default for PlayerData {
    fn default() -> PlayerData {
        PlayerData {
            energy: 100,
            gold: 50,
            exp: 0,
            id: uuid::Uuid::new_v4(),
            last_updated: chrono::Utc::now().naive_utc(),
            gold_acc: 10,
            player_stats_id: uuid::Uuid::new_v4(),
            player_inventory_id: uuid::Uuid::new_v4(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Insertable)]
#[table_name = "player_factories"]
pub struct PlayerFactories {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub factory_id: uuid::Uuid,
    pub amount: i32,
}

impl Default for PlayerFactories {
    fn default() -> PlayerFactories {
        PlayerFactories {
            id: uuid::Uuid::new_v4(),
            user_id: uuid::Uuid::new_v4(),
            factory_id: uuid::Uuid::new_v4(),
            amount: 0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Insertable)]
#[table_name = "player_inventory"]
pub struct PlayerInventory {
    pub id: uuid::Uuid,
    pub capacity: i32,
    pub food_q1: i32,
    pub weapon_q1: i32,
    pub special_currency: i32,
}

impl Default for PlayerInventory {
    fn default() -> PlayerInventory {
        PlayerInventory {
            id: uuid::Uuid::new_v4(),
            capacity: 100,
            food_q1: 10,
            weapon_q1: 0,
            special_currency: 0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Insertable)]
#[table_name = "player_stats"]
pub struct PlayerStats {
    pub id: uuid::Uuid,
    pub strength: i32,
    pub agility: i32,
    pub stamina: i32,
}

impl Default for PlayerStats {
    fn default() -> PlayerStats {
        PlayerStats {
            id: uuid::Uuid::new_v4(),
            strength: 1,
            agility: 1,
            stamina: 1,
        }
    }
}
