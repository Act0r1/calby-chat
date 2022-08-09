table! {
    chat (chat_id) {
        chat_id -> Int8,
        room -> Int8,
        creator -> Varchar,
        displayed_name -> Varchar,
        short_name -> Varchar,
        chat_type -> Varchar,
        avatar -> Text,
        users -> Array<Text>,
        open -> Nullable<Bool>,
        description -> Nullable<Text>,
    }
}

table! {
    messages (msg_id) {
        msg_id -> Int8,
        chat_id -> Int8,
        content -> Text,
        author -> Varchar,
        time -> Int8,
        who_received -> Text,
        who_read -> Text,
    }
}

joinable!(messages -> chat (chat_id));

allow_tables_to_appear_in_same_query!(
    chat,
    messages,
);
