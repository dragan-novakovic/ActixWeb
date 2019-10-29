table! {
    lots (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Text>,
        price -> Int4,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        username -> Varchar,
        password -> Varchar,
        created_on -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    lots,
    users,
);
