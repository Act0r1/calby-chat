table! {
    chats (chat_id) {
        chat_id -> Int8,
        room -> Int8,
        creator -> Nullable<Varchar>,
        displayed_name -> Nullable<Varchar>,
        short_name -> Nullable<Varchar>,
        chat_type -> Nullable<Varchar>,
        avatar -> Nullable<Text>,
        users -> Array<Text>,
        open -> Nullable<Bool>,
        description -> Nullable<Text>,
    }
}

table! {
    messages (msg_id) {
        msg_id -> Int8,
        chat_id -> Int8,
        content -> Nullable<Text>,
        author -> Nullable<Varchar>,
        time -> Nullable<Timestamptz>,
        who_received -> Nullable<Text>,
        who_read -> Nullable<Text>,
    }
}

joinable!(messages -> chats (chat_id));

allow_tables_to_appear_in_same_query!(
    chats,
    messages,
);
