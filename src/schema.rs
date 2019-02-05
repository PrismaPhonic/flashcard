table! {
    cards (id) {
        id -> Int4,
        question -> Text,
        answer -> Text,
        deck_id -> Int4,
    }
}

table! {
    decks (id) {
        id -> Int4,
        title -> Text,
        created_at -> Timestamp,
        author -> Text,
    }
}

table! {
    users (username) {
        username -> Text,
        password -> Text,
    }
}

joinable!(cards -> decks (deck_id));
joinable!(decks -> users (author));

allow_tables_to_appear_in_same_query!(
    cards,
    decks,
    users,
);
