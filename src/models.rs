use diesel::pg::Pg;
use diesel::types::{FromSql,ToSql};
use serde::{Deserialize, Serialize};
extern crate serde_json;
use crate::schema::{chats, messages};
use diesel::sql_types::Jsonb;
use diesel::Insertable;
#[derive( Queryable, PartialEq, Debug, Deserialize, Serialize)]
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
#[derive(Queryable, Associations, PartialEq, Debug)]
pub struct Messages {
    pub id: i64,
    pub author: String,
    pub content: String,
    pub time: String,
    pub who_received: String,
    pub who_read: String,
}

#[derive(Debug,FromSqlRow, Insertable, Serialize, Deserialize)]
#[table_name="chats"]
#[diesel(sql_type = "Jsonb")]
pub struct NewGroup {
   pub creator: String,
   pub displayed_name: String,
   pub chat_type: String,
   pub avatar: String,
   pub users: Vec<String>,
   pub open: bool,
   pub description: String,
}

#[derive(Debug, AsExpression,FromSqlRow, Insertable, Serialize, Deserialize)]
#[table_name="chats"]
#[diesel(sql_type = "Jsonb")]
pub struct NewChat {
   pub creator: String,
   pub displayed_name: String,
   pub chat_type: String,
   pub avatar: String,
   pub users: Vec<String>,
   pub open: bool,
   pub description: String,
}


//impl FromSql<Vec<T>, Pg>  for Vec<String> {
//    fn from_sql() {}
//}
