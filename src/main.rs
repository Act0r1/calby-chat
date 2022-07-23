//use chats::chats_server::{ChatsServer, Chats};
#[macro_use]
extern crate diesel;


//pub mod dbpool;
pub mod schema;
pub mod gRPC;
pub mod models;
use std::env;
use tonic::transport::Server;
//static DATABASE_URL: &'static str = "postgresql://calby:error@localhost:5432/calbychat";
use gRPC::chats::chats_server::ChatsServer;


type MResult<T> = Result<T, Box<dyn std::error::Error>>;
#[tokio::main]
async fn main() ->MResult<()>{
    dotenv::dotenv()?;
    Ok(())
}


