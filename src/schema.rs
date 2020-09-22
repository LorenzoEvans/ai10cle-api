table! {
    articles (user_id) {
        id -> Int4,
        user_id -> Int4,
        title -> Text,
        link -> Text,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    articles,
    users,
);
