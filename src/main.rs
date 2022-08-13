#[macro_use]
extern crate diesel;
//pub mod dbpool;
pub mod schema;
pub mod g_rpc;
pub mod models;
use tonic::transport::Server;
static DATABASE_URL: & str = "postgresql://calby:error@localhost:5432/calbychat";
use g_rpc::calby_chat::calby_chat_server::CalbyChatServer;
use g_rpc::ChatService;
pub mod calby_chat {
    tonic::include_proto!("calby_chat");
}

type MResult<T> = Result<T, Box<dyn std::error::Error>>;
#[tokio::main]
async fn main() -> MResult<()>{
    let addr: std::net::SocketAddr = "[::1]:50051".parse().expect("Can't parse'");
    let manager = bb8_diesel
                  ::DieselConnectionManager
                  ::<diesel::pg::PgConnection>
                  ::new(DATABASE_URL); 
    let pool = bb8::Pool::builder().max_size(15).build(manager).await?;
    let users = ChatService { db: pool };
    let svc = CalbyChatServer::new(users);
    Server::builder().add_service(svc).serve(addr).await?;
    Ok(())
}


