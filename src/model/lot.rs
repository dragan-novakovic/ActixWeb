#[derive(Clone,Debug, Serialize, Deserialize, PartialEq, Queryable)]
pub struct Lot {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: i32
}