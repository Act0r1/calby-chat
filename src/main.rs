use chats::chats_server::ChatsServer;


pub mod chats {
    tonic::include_proto!("chats");
}

fn main() {
    println!("Hello, world!");
}
