use crate::schema::lots;
use uuid::Uuid;

#[derive(Clone,Debug, Serialize, Deserialize, PartialEq, Queryable, Insertable)]
#[table_name = "lots"]
pub struct Lot {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: i32
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NewLot {
    pub name: String,
    pub description: String,
    pub price: i32
}