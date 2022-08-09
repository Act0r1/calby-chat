use diesel::prelude::*;
use std::time::SystemTime;
use prost_types::Timestamp;
use chrono::{DateTime, Utc, NaiveDateTime};
// use crate::models::Message;
use crate::{models::*, schema::*};
use diesel::pg::PgConnection;
use tonic::{transport::Server, Request, Response, Status};
pub mod calby_chat {
    tonic::include_proto!("calby_chat");
}
use tonic::Code;
// methods from proto
use self::calby_chat::{
    calby_chat_server::CalbyChat, GroupSettings, Message, MsgForDel, MsgForEdit, ResponseMessage,ResponseGroup,
};
/// Структура, содержащая данные нашего микросервиса.
#[derive(Clone)]
pub struct ChatService {
  pub db: bb8::Pool<bb8_diesel::DieselConnectionManager<diesel::pg::PgConnection>>,
}
#[tonic::async_trait]
impl CalbyChat for ChatService {
    // create db connection
    async fn create_group(
        &self,
        request: Request<GroupSettings>,
    ) -> Result<Response<ResponseGroup>, Status> {
    use crate::schema::chat;
    let reply = ResponseGroup {
           status: true};
    let conn = self.db.get().await.unwrap();
    let GroupSettings {creator, displayed_name,
    short_name, chat_type, avatar, users, open, description} = request.into_inner(); 
    let new_group = NewGroup{
        creator,
        displayed_name,
        short_name,
        chat_type,
        avatar,
        users:vec![users],
        open,
        description,
    };
    diesel::insert_into(chat::table)
    .values(&new_group)
    .execute(&*conn)
    .unwrap();   
    Ok(Response::new(reply))

  }
    async fn send_message(
        &self,
        request: Request<Message>,
    ) -> Result<Response<ResponseMessage>, Status> {
        let reply = ResponseMessage{
            status:true };
        use crate::schema::messages;
        // let dt = DateTime:: 
        let conn = self.db.get().await.unwrap();
        let Message {chat_id, author, content, time, who_received, who_read} = request.into_inner();
        let msg = Messages {
            chat_id,
            author,
            content,
            // this is 'weird' way to convert prost_types::Timestamp for NaiveDateTime
            // time:NaiveDateTime::from_timestamp(0,time.unwrap_or_default().nanos as u32),
            time,
            who_received,
            who_read,
        };
        diesel::insert_into(messages::table)
            .values(&msg)
            .execute(&*conn)
            .unwrap();
        Ok(Response::new(reply))
    }

    async fn edit_message(
        &self,
        request: Request<MsgForEdit>,
    ) -> Result<Response<ResponseMessage>, Status> {
        unimplemented!();
    }

    async fn delete_message(
        &self,
        request: Request<MsgForDel>,
    ) -> Result<Response<ResponseMessage>, Status> {
        unimplemented!();
    }
}

