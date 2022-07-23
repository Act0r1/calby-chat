extern crate diesel;
//use tokio_postgres::Connection;
use diesel::pg::PgConnection;
use tonic::{transport::Server, Request, Response, Status};
pub mod chats {
    tonic::include_proto!("chats");
}
// methods from proto
use chats::{
    chats_server::Chats, GroupSettings, Message, MsgForDel, MsgForEdit, ResponseMessage,ResponseGroup,
};



/// Структура, содержащая данные нашего микросервиса.
#[derive(Clone)]
pub struct ChatsService {
  pub db: bb8::Pool<bb8_diesel::DieselConnectionManager<diesel::pg::PgConnection>>,
}

#[tonic::async_trait]
impl Chats for ChatsService {
    // create db connection
    async fn create_group(
        &self,
        _request: Request<GroupSettings>,
    ) -> Result<Response<ResponseGroup>, Status> {
        let pool = dbpool::connect();
        let reply = ResponseGroup {
            status: 200,
            error: "No error".to_string(),
            uuid: "asdsad".to_string()
        };
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

