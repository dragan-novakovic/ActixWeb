#[derive(Clone,Debug, Serialize, Deserialize, PartialEq)]
pub struct Lot {
    pub id: i32,
    pub name: String,
    pub price: i32
}

impl Lot {
    pub fn new() -> Lot {
        Lot {
            id: 0,
            name: "".to_string(),
            price: 0
        }
    }
}