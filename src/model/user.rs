use crate::schema::{players_data, users};
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
}

impl User {
    pub fn remove_pwd(mut self) -> Self {
        self.password = "".to_string();
        self
    }

    pub fn from_details<S: Into<String>, T: Into<String>, V: Into<String>>(
        email: S,
        pwd: T,
        username: V,
    ) -> Self {
        User {
            id: uuid::Uuid::new_v4(),
            email: email.into(),
            username: username.into(),
            password: pwd.into(),
            created_on: chrono::Local::now().naive_local(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Insertable)]
#[table_name = "players_data"]
pub struct PlayerData {
    pub energy: i32,
    pub gold: i32,
    pub exp: i32,
    pub user_id: uuid::Uuid,
}
