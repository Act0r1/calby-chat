use diesel::{prelude::*, types::ToSql, sql_types::Integer, pg::Pg};
// use crate::models::Message;
use crate::{models::*, schema::*};
use tonic::{transport::Server, Request, Response, Status};
pub mod calby_chat {
    tonic::include_proto!("calby_chat");
}
// methods from proto
use self::calby_chat::{
    calby_chat_server::CalbyChat, GroupSettings, Message,ResponseMessage,ResponseGroup,
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
        users,
        open,
        description,
    };
    diesel::insert_into(chat::table)
    .values(new_group)
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
        request: Request<Message>,
    ) -> Result<Response<ResponseMessage>, Status> {
        let reply = ResponseMessage{
            status:true };
        use crate::schema::messages;
        // let dt = DateTime:: 
        let conn = self.db.get().await.unwrap();
        let Message {
            chat_id,
            author:_,
            content,
            // this is 'weird' way to convert prost_types::Timestamp for NaiveDateTime
            // time:NaiveDateTime::from_timestamp(0,time.unwrap_or_default().nanos as u32),
            time,
            who_received,
            who_read,
        } = request.into_inner();
        diesel::update(messages::table.filter(messages::chat_id.eq(&chat_id)))
            .set((messages::content.eq(content), 
                    messages::time.eq(time), 
                    messages::who_read.eq(who_read),
                    messages::who_received.eq(who_received)))
            .execute(&*conn)
            .unwrap();
        Ok(Response::new(reply))
    }

    async fn delete_message(
        &self,
        request: Request<Message>,
    ) -> Result<Response<ResponseMessage>, Status> {
        let reply = ResponseMessage{
            status:true };
        use crate::schema::messages;
        // let dt = DateTime:: 
        let conn = self.db.get().await.unwrap();
        let Message {
            chat_id,
            author,
            content,
            time:_,
            who_received:_,
            who_read:_,
        } = request.into_inner();
        // let msg = Messages {
        //     chat_id,
        //     author,
        //     content,
        //     // this is 'weird' way to convert prost_types::Timestamp for NaiveDateTime
        //     // time:NaiveDateTime::from_timestamp(0,time.unwrap_or_default().nanos as u32),
        //     time,
        //     who_received,
        //     who_read,
        // };
        // let source = messages::table
        //     .filter(messages::author.eq(&author))
        //     .load(&*conn);
        diesel::delete(messages::table
            .filter(messages::chat_id.eq(&chat_id))
            .filter(messages::author.eq(&author)))
            .filter(messages::content.eq(&content))
            .execute(&*conn)
            .unwrap();
        // diesel::delete(messages::table
        //     .filter(chat_id.eq(&chat_id))
        //     .filter(content.eq(&content))
        //
        //     ) 
        //             
        // .execute(&*conn)
        // .unwrap();
        Ok(Response::new(reply))
    }
}
