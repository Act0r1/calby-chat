use diesel::Queryable;
#[derive(Queryable)]
pub struct Chats{
   pub chat_id: i64,
   pub room: i64,
   pub creator: String,
   pub displayed_name: String,
   pub chat_type: String,
   pub avatar: String,
   pub users: Vec<String>,
   pub open: bool,
   pub description: String,
}
#[derive(Queryable)]
pub struct Messages{
    pub id: i64,
    pub author: String,
    pub content: String,
    pub time: String,
    pub who_received: String,
    pub who_read: String,
}
