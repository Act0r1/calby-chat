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
        users: vec![123,213,123,34543],
        open:true,
        description: "I done, yoww".into()
    });
    let request2 = Request::new(Message{
        chat_id:1,
        content:"This msg should be delete?".into(),
        author:"Act0r".into(),
        time:234328979,
        who_read:vec![123,213,123,34543],
        who_received:vec![234,549,4562]
    });
    let request3 = Request::new(Message{
        chat_id:1,
        content:"Hello Insaf, content change??".into(),
        author:"Act0r".into(),
        time:111111,
        who_read:vec![450,345342,2435],
        who_received:vec![435,2345,2435,4356]
    });

    let request5 = Request::new(Message{
        chat_id:1,
        content:"This msg not be delete?".into(),
        author:"Act0r".into(),
        time:111111,
        who_read:vec![450,345342,2435],
        who_received:vec![435,2345,2435,4356]
    });
    let request4 = Request::new(Message{
        chat_id:1,
        content:"This msg should be delete?".into(),
        author:"Act0r".into(),
        time:111111,
        who_read:vec![450,345342,2435],
        who_received:vec![435,2345,2435,4356]
    });
    let response3 = client.edit_message(request3).await?;
    let response = client.create_group(request).await?;
    let response2 = client.send_message(request2).await?;
    let response4 = client.delete_message(request4).await?;
    println!("RESPONSE={:?}", response);
    println!("Response={:?}", response3);
    println!("RESPONSE={:?}", response4);
    println!("RESPONSE={:?}", response2);
    Ok(())
}
