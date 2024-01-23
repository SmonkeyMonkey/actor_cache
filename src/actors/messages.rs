use serde::Serialize;
use std::env;

#[derive(Debug, Serialize)]
pub enum MessageType {
    INPUT,
    OUTPUT,
    EMPTY,
}

/// This struct defines messages being sent to the state actor.
/// * message type: the type of message,if we are sending an input or output message
/// * chat_id: the ID of the chat in which we update te data
/// * single_data: if input, this field will be populated to be insertted
/// * block_data: if output, the entire chat history of the user to be sent to the librarian server
#[derive(Debug, Serialize)]
pub struct StateActorMessage {
    pub message_type: MessageType,
    pub chat_id: Option<i32>,
    pub single_data: Option<String>,
    pub block_data: Option<Vec<String>>,
}


impl StateActorMessage {
    ///  Sends ```self.block_data``` to another server.
    /// * Use  ```export SERVER_URL="your_url"```
    /// * For example hard-coded into https://httpbin.org/post
    pub async fn send_to_server(&self) {
        let lib_url = "https://httpbin.org/post";
        // let lib_url = env::var("SERVER_URL").unwrap();
        let joined = self.block_data.clone().unwrap().join("$");
        let body = PostBody {
            chat_id: self.chat_id.unwrap(),
            block_data: joined,
        };
        let client = reqwest::Client::new();
        let res = client.post(lib_url).json(&body).send().await.unwrap();
        println!("{:?}", res);
    }
}

#[derive(Debug, Serialize)]
pub struct PostBody {
    pub chat_id: i32,
    pub block_data: String,
}
