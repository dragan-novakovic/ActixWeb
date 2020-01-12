use crate::schema::users;
use chrono::prelude::*;
use uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub username: String,
    pub password: String,
    pub created_on: NaiveDateTime,
    pub player_data_id: uuid::Uuid,
}

impl User {
    pub fn remove_pwd(mut self) -> Self {
        self.password = "".to_string();
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}
// #[derive(Debug, Serialize, Deserialize)]
// struct UserWithData {
//     pub id: uuid::Uuid,
//     pub email: String,
//     pub username: String,
//     pub password: String,
//     pub player_data: PlayerData,
// }

// impl
//     From<(
//         uuid::Uuid,
//         String,
//         String,
//         String,
//         uuid::Uuid,
//         i32,
//         i32,
//         i32,
//         NaiveDateTime,
//         i32,
//         uuid::Uuid,
//         uuid::Uuid,
//     )> for UserWithData
// {
//     fn from(
//         tup: (
//             uuid::Uuid,
//             String,
//             String,
//             String,
//             uuid::Uuid,
//             i32,
//             i32,
//             i32,
//             NaiveDateTime,
//             i32,
//             uuid::Uuid,
//             uuid::Uuid,
//         ),
//     ) -> UserWithData {
//         UserWithData {
//             id: tup.0,
//             email: tup.1,
//             username: tup.2,
//             password: tup.3,
//             player_data: PlayerData {
//                 id: tup.4,
//                 energy: tup.5,
//                 gold: tup.6,
//                 exp: tup.7,
//                 last_updated: tup.8,
//                 gold_acc: tup.9,
//                 player_stats_id: tup.10,
//                 player_inventory_id: tup.11,
//             },
//         }
//     }
// }

// impl UserWithData {
//     fn remove_pass(mut self) -> Self {
//         self.password = "".to_owned();
//         self
//     }

//     fn update_gold(mut self) -> Self {
//         self.player_data.gold = self.player_data.gold
//             + get_current_time_diff(self.player_data.last_updated) * self.player_data.gold_acc;
//         self
//     }

//     fn new_gold(&self) -> i32 {
//         self.player_data.gold
//             + get_current_time_diff(self.player_data.last_updated) * self.player_data.gold_acc
//     }
// }

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserId {
    pub id: uuid::Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserInventoryId {
    pub inventory_id: uuid::Uuid,
}
