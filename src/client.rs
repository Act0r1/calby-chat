pub mod calby_chat {
    tonic::include_proto!("calby_chat");
}
use log::info;
use tonic::Request;
use calby_chat::calby_chat_client::CalbyChatClient;
use self::calby_chat::{
     GroupSettings,
     Message,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CalbyChatClient::connect("http://[::1]:50051").await?;
    let request = Request::new(GroupSettings {
        creator: "Actor".into(),
        displayed_name: "Actor".into(),
        short_name: "Act".into(),
        chat_type: "group".into(),
        avatar: "/usr/img/source.png".into(),
        users: "asdasd".into(),
        open:true,
        description: "I done, yoww".into()
    });
    let request2 = Request::new(Message{
        chat_id:1,
        content:"Hello Insaf, how are you?".into(),
        author:"Act0r".into(),
        time:234328979,
        who_read:"asdasd".into(),
        who_received:"all".into()
    });

    let response = client.create_group(request).await?;
    let response2 = client.send_message(request2).await?;
    info!("RESPONSE={:?}", response);
    info!("RESPONSE={:?}", response2);
    Ok(())
}
