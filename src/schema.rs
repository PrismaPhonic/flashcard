table! {
    cards (id) {
        id -> Int4,
        question -> Text,
        answer -> Text,
        deck_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    decks (id) {
        id -> Int4,
        title -> Text,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
    }
}

joinable!(cards -> decks (deck_id));
joinable!(cards -> users (user_id));

allow_tables_to_appear_in_same_query!(
    cards,
    decks,
    users,
);
