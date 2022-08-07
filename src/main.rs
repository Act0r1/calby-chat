#[macro_use]
extern crate diesel;
//pub mod dbpool;
pub mod schema;
pub mod g_rpc;
pub mod models;
use tonic::transport::Server;
static DATABASE_URL: & str = "postgresql://calby:error@localhost:5432/calbychat";
use g_rpc::chat::chat_server::ChatServer;
use g_rpc::ChatService;
pub mod chat {
    tonic::include_proto!("chat");
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
    let svc = ChatServer::new(users);
    Server::builder().add_service(svc).serve(addr).await?;
    Ok(())
}


