table! {
    lots (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Text>,
        price -> Int4,
    }
}
