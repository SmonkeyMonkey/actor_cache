use std::collections::{VecDeque, HashMap};
// use std::mem;

use tokio::sync::mpsc::{Receiver,Sender};

use crate::actors::messages::MessageType;

use super::messages::StateActorMessage;

/// This actor is responsible for managing the state of all the chat logs for all chat IDs.
/// * chat_queue: the order in which the chats were inserted into the state
/// * chats_logs: the chat logs in relation to the patient
/// * receiver: the receiver that accepts INPUT and OUTPUT messages through a channel
/// * sender: the sender that sends messages through a channel
#[derive(Debug)]
pub struct StateActor {
    pub chat_queue: VecDeque<i32>,
    pub chat_logs: HashMap<i32,Vec<String>>,
    pub receiver: Receiver<StateActorMessage>,
    pub sender: Sender<StateActorMessage>
}

impl StateActor {
    pub fn new(receiver: Receiver<StateActorMessage>,sender: Sender<StateActorMessage>) -> StateActor{
        StateActor{ chat_queue: VecDeque::new(), chat_logs: HashMap::new(), receiver, sender }
    }
    pub fn get_message_data(&mut self,chat_id: i32)-> Vec<String> {
        self.chat_logs.remove(&chat_id).unwrap()
    }
    pub fn insert_message(&mut self,chat_id: i32,message: String){
        match self.chat_logs.get_mut(&chat_id) {
            Some(log) => {
                log.push(message);
            },
            None => {
                self.chat_queue.push_back(chat_id);
                self.chat_logs.insert(chat_id, vec![message]);
            },
        }
    }
    pub async fn handle_message(&mut self,message: StateActorMessage){
        println!("StateActor receiving a message");

        match message.message_type {
            MessageType::INPUT => {
                self.insert_message(message.chat_id.unwrap(), message.single_data.unwrap());
            },
            MessageType::OUTPUT => {
                match self.chat_queue.pop_front() {
                    Some(chat_id) => {
                        let data = self.get_message_data(chat_id);
                        let message = StateActorMessage{
                            message_type: MessageType::OUTPUT,
                            chat_id: Some(chat_id),
                            single_data: None,
                            block_data: Some(data),
                        };
                        let _ = self.sender.send(message).await.unwrap();
                    },
                    None => {
                        let message = StateActorMessage {
                            message_type: MessageType::EMPTY,
                            chat_id: None,
                            single_data: None,
                            block_data: None,
                        };
                        let _ = self.sender.send(message).await.unwrap();
                    },
                }
            },
            MessageType::EMPTY => {
                panic!("empty message should not be sent to the state actor")
            },
        }
        println!("{:?}",self.chat_logs);
        println!("{:?}",self.chat_queue);
    }
    pub async fn run(mut self){
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg).await;
        }
    }
}