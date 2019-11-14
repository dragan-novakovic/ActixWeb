table! {
    factories (id) {
        id -> Uuid,
        level -> Int4,
        gold_per_day -> Int4,
        price -> Int4,
    }
}

table! {
    invitations (id) {
        id -> Uuid,
        email -> Varchar,
        expires_at -> Timestamp,
    }
}

table! {
    player_factories (user_id, factory_id) {
        user_id -> Uuid,
        factory_id -> Uuid,
        amount -> Int4,
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

joinable!(player_factories -> factories (factory_id));
joinable!(player_factories -> users (user_id));
joinable!(players_data -> users (user_id));

allow_tables_to_appear_in_same_query!(
    factories,
    invitations,
    player_factories,
    players_data,
    users,
);
