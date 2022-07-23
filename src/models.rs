use diesel::Queryable;
use crate::schema::{chats, messages};
use diesel::Insertable;
#[derive( Queryable, PartialEq, Debug)]
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
#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Chats)]
#[table_name = "chats"]
pub struct Messages {
    pub id: i64,
    pub author: String,
    pub content: String,
    pub time: String,
    pub who_received: String,
    pub who_read: String,
}

#[derive(Debug, Insertable)]
#[table_name="chats"]
pub struct NewGroup<'a> {
   pub creator: String,
   pub displayed_name: String,
   pub chat_type: String,
   pub avatar: String,
   pub users: &'a Vec<String>,
   pub open: bool,
   pub description: String,
}

#[derive(Debug, Insertable)]
#[table_name="chats"]
pub struct NewChat<'a> {
   pub creator: String,
   pub displayed_name: String,
   pub chat_type: String,
   pub avatar: String,
   pub users: &'a Vec<String>,
   pub open: bool,
   pub description: String,
}
