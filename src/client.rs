pub mod chat {
    tonic::include_proto!("chat");
}
use chat::chat_client::ChatClient;
use self::chat::{
     GroupSettings
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ChatClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(GroupSettings {
        creator: "Actor".into(),
        displayed_name: "Actor".into(),
        short_name: "Act".into(),
        chat_type: "group".into(),
        avatar: "/usr/img/source.png".into(),
        users: vec!["sad".to_string(), "asd".to_string(), "sdf".to_string()],
        open:true,
        description: "I done, yoww".into()
    });
    let response = client.create_group(request).await?;
    println!("RESPONSE={:?}", response);

    Ok(())
}
