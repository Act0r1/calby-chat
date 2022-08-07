use diesel::prelude::*;
use crate::{models::*, schema::*};
use diesel::pg::PgConnection;
use tonic::{transport::Server, Request, Response, Status};
pub mod chat {
    tonic::include_proto!("chat");
}
use tonic::Code;
// methods from proto
use self::chat::{
    chat_server::Chat, GroupSettings, Message, MsgForDel, MsgForEdit, ResponseMessage,ResponseGroup,
};
/// Структура, содержащая данные нашего микросервиса.
#[derive(Clone)]
pub struct ChatService {
  pub db: bb8::Pool<bb8_diesel::DieselConnectionManager<diesel::pg::PgConnection>>,
}
#[tonic::async_trait]
impl Chat for ChatService {
    // create db connection
    async fn create_group(
        &self,
        request: Request<GroupSettings>,
    ) -> Result<Response<ResponseGroup>, Status> {
    use crate::schema::chats;
    let reply = ResponseGroup {
           status: 200,
           error:"No error".to_string(),};
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
    diesel::insert_into(chats::table)
    .values(&new_group)
    .execute(&*conn)
    .unwrap();   
    Ok(Response::new(reply))

  }
    async fn send_message(
        &self,
        request: Request<Message>,
    ) -> Result<Response<ResponseMessage>, Status> {
        unimplemented!();
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

//#[tokio::main]

