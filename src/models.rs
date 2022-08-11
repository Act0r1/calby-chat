extern crate serde_json;
use serde::{Deserialize, Serialize};
use crate::schema::{chat, messages};
use diesel::{Insertable, Identifiable};
#[derive(Identifiable,Queryable, PartialEq, Debug, Deserialize, Serialize)]
#[primary_key(chat_id)]
#[table_name="chat"]
pub struct Chats {
   pub chat_id: i64,
   pub room: i64,
   pub creator: String,
   pub displayed_name: String,
   pub chat_type: String,
   pub avatar: String,
   pub users: Vec<i32>,
   pub open: bool,
   pub description: String,
}
#[derive(Serialize, Deserialize, Insertable, Associations,Debug)]
#[table_name="messages"]
#[belongs_to(Chats, foreign_key="chat_id")]
pub struct Messages {
   pub chat_id: i64,
   pub author: String,
   pub content: String,
   pub time:i64,
   pub who_received: Vec<i32>,
   pub who_read: Vec<i32>,
}

// #[derive(Insertable, FromSqlRow, Serialize, Deserialize,Debug)]
// // #[primary_key(chat_id)]
// #[table_name="messages"]
// pub struct Msg {
//    pub chat_id: i64,
//    pub author: String,
//    pub content: String,
//    pub time:i64,
//    pub who_received: String,
//    pub who_read: String,
// }

#[derive(Debug,FromSqlRow, Insertable, Serialize, Deserialize)]
#[table_name="chat"]
// #[diesel(sql_type = "Jsonb")]
pub struct NewGroup {
   pub creator: String,
   pub displayed_name: String,
   pub short_name: String,
   pub chat_type: String,
   pub avatar: String,
   pub users: Vec<i32>,
   pub open: bool,
   pub description: String,
}
//
// #[derive(Debug, AsExpression,FromSqlRow, Insertable, Serialize, Deserialize)]
// #[table_name="chat"]
// #[diesel(sql_type = "Jsonb")]
// pub struct NewChat {
//    pub creator: String,
//    pub displayed_name: String,
//    pub chat_type: String,
//    pub avatar: String,
//    pub users: Vec<String>,
//    pub open: bool,
//    pub description: String,
// }
//
// #[derive(Debug, AsExpression,FromSqlRow, Insertable, Serialize, Deserialize)]
// #[table_name="messages"]
// pub struct EditMsg {
//    pub chat_id: i64,
//    pub author: String,
//    pub msg_id: i64,
//    pub content: String,
// }
