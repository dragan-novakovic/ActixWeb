use crate::schema::factories;
use uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Insertable)]
#[table_name = "factories"]
pub struct Factory {
    pub id: uuid::Uuid,
    pub level: i32,
    pub gold_per_day: i32,
    pub price: i32,
    pub name: String,
}
