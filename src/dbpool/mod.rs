//use diesel::pg::PgConnection;
//use tonic::{transport::Server, Request, Response, Status};
//
//// methods from proto
//use crate::gRPC::chats::GroupSettings;
//
//
//pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
//static DATABASE_URL: &'static str = "postgresql://calby:error@localhost:5432/calbychat";
//
//pub fn connect() -> Pool {
//    let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL);
//    r2d2::Pool::builder().build(manager).expect("Failed to create pool")
//}
//
//pub async fn create_group(_req:Request<GroupSettings>) -> Result<String,Status> {
//    let pool = connect();
//    let conn = pool.get().unwrap(); 
//    let GroupSettings {id, room, creator, displayed_name,
//    short_name, chat_type, avatar, users, open, desc} = &req.into_inner();
//    let create_group = &conn.execute("CREATE IF NOT EXISTS ");
//    Ok(String::from("calby"))
//
//}
//
