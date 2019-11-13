table! {
    invitations (id) {
        id -> Uuid,
        email -> Varchar,
        expires_at -> Timestamp,
    }
}

table! {
    lots (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Text>,
        price -> Int4,
    }
}

table! {
    players_data (user_id) {
        energy -> Int4,
        gold -> Int4,
        exp -> Int4,
        user_id -> Uuid,
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

joinable!(players_data -> users (user_id));

allow_tables_to_appear_in_same_query!(
    invitations,
    lots,
    players_data,
    users,
);
