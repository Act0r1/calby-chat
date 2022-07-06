//use chats::chats_server::{ChatsServer, Chats};
use r2d2;
extern crate diesel;
pub mod dbpool;
use r2d2_diesel::ConnectionManager;
//use tokio_postgres::Connection;
use diesel::pg::PgConnection;
use tonic::{transport::Server, Request, Response, Status};

pub mod chats {
    tonic::include_proto!("chats");
}

// methods from proto
use crate::chats::{
    chats_server::Chats, GroupSettings, Message, MsgForDel, MsgForEdit, ResponseMessage,ResponseGroup,
};

// types from prot

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Default)]
pub struct Chat {}

#[tonic::async_trait]
impl Chats for Chat {
    // create db connection
    async fn create_group(
        &self,
        request: Request<GroupSettings>,
    ) -> Result<Response<ResponseGroup>, Status> {
        let pool = dbpool::connect();
        let reply = ResponseGroup {
            status: 200,
            error: "No error".to_string()
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
fn main() {
    println!("Hello, world!");
}
