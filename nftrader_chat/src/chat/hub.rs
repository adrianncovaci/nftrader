use std::time::Duration;
use futures::StreamExt;
use tokio_stream::wrappers::UnboundedReceiverStream;
use regex::Regex;
use tokio::{sync::{broadcast, mpsc::UnboundedReceiver}, time::error::Error};
use uuid::Uuid;
use lazy_static::lazy_static;
use crate::{GlobalFeed, Users, models::{message::Message, proto::{ErrorType, Input, InputParcel, JoinedInput, JoinedOutput, Output, OutputParcel, PostedInput, PostedOutput, SelfJoinedOutput}, user::User}};

const MAX_CLIENTS_ALLOWED: usize = 16;
const MAX_MESSAGE_BODY_LENGTH: usize = 64;
lazy_static! {
    static ref REGEX_NAME_VALIDATOR: Regex = Regex::new("^[A-Za-z0-9_-]{4, 20}").unwrap();
}

#[derive(Debug, Clone)]
pub struct Hub {
    sender: broadcast::Sender<OutputParcel>,
    users: Users,
    feed: GlobalFeed,
    alive_interval: Option<Duration>
}

impl Hub {
    pub fn new(alive_interval: Option<Duration>) -> Self {
        let (tx, _) = broadcast::channel(MAX_CLIENTS_ALLOWED);

        Self {
            alive_interval,
            feed: Default::default(),
            users: Default::default(),
            sender: tx
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<OutputParcel> {
        self.sender.subscribe()
    }

    pub async fn broadcast(&self, output: Output) {
        if self.sender.receiver_count() == 0 {
            return;
        }

        self.users.read().await.iter().for_each(move |user| {
            self.sender.send(
                OutputParcel::new(
                    user.get_id(),
                    output.clone()
                )
            ).unwrap();
        });
    }

    pub async fn send_directed(&self, output: Output, client_id: Uuid) {
        if self.sender.receiver_count() == 0 {
            return;
        }

        self.sender.send(
            OutputParcel::new(
                client_id,
                output
        )).unwrap();
    }

    pub async fn send_ignored(&self, output: Output, client_id: Uuid) {
        self.users.read().await.iter()
            .filter(|usr| usr.get_id() != client_id)
            .for_each(|usr| {
                self.sender.send(OutputParcel::new(usr.get_id(), output.clone())).unwrap();
            });
    }

    pub async fn send_error(&self, error: ErrorType, client_id: Uuid) {
        self.send_directed(Output::Error(error), client_id).await;
    }

    pub async fn disconnect_user(&self, client_id: Uuid) {
        let result = self.users.read().await.iter().position(|usr| usr.get_id() == client_id);
        if let Some(index) = result {
            self.users.write().await.remove(index);
        }
    }

    pub async fn process(&self, input: InputParcel) {
        match input.input {
            Input::Joined(msg) => self.process_join(msg, input.client_id).await,
            Input::Posted(msg) => self.process_post(msg, input.client_id).await,
        }
    }

    pub async fn process_join(&self, input: JoinedInput, client_id: Uuid) {
        if self.users.read().await.iter().find(|usr| usr.get_nickname() == input.nickname).is_some() {
            self.send_error(ErrorType::NameTaken, client_id).await;
            return;
        }

        if !REGEX_NAME_VALIDATOR.is_match(input.nickname.as_str()) {
            self.send_error(ErrorType::InvalidName, client_id).await;
            return;
        }

        let user = User::new(client_id, &input.nickname);
        self.users.write().await.push(user.clone());
        println!("client {} joined", client_id);
        self.send_directed(Output::SelfJoined(SelfJoinedOutput{ nickname: user.get_nickname(), client_id }), client_id).await;
        self.send_ignored(Output::Joined(JoinedOutput{ nickname: user.get_nickname() }), client_id).await;
    }

    pub async fn process_post(&self, input: PostedInput, client_id: Uuid) {
        let user;
        match self.users.read().await.iter().find(|usr| usr.get_id() == client_id) {
            Some(usr) => user = usr.clone(),
            None => {
                self.send_error(ErrorType::NotJoined, client_id).await;
                return;
            }
        };

        if input.content.len() > MAX_MESSAGE_BODY_LENGTH || input.content.is_empty() {
            self.send_error(ErrorType::InvalidContent, client_id).await;
            return;
        }
        
        let message = Message::new(user.get_id(), input.content.as_str());
        self.feed.write().await.add_message(message); 

        self.send_ignored(Output::Posted(PostedOutput { content: input.content }), client_id).await;
    }

    pub async fn run(&self, rx: UnboundedReceiver<InputParcel>) {
        let rx = UnboundedReceiverStream::new(rx);
        rx.for_each(|input| self.process(input)).await;
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::Arc;

    use tokio::sync::RwLock;

    use crate::models::user::User;

    use super::*;
    #[tokio::test]
    async fn test_broadcasting() {
        //replace user with client

        let vanea = User::new(Uuid::new_v4(), "vanea");
        let users = Arc::new(RwLock::new(vec![vanea.clone()]));
        let feed: GlobalFeed = Default::default();
        let (tx, _) = broadcast::channel(MAX_CLIENTS_ALLOWED);
        let hub = Hub { alive_interval: None, feed, sender: tx, users};
        let mut rx1 = hub.subscribe();
        let mut rx2 = hub.subscribe();

        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        let vanea_clone = vanea.clone();
        tokio::spawn(async move {
            tx.send(InputParcel { client_id: vanea_clone.clone().get_id(), input: Input::Joined(JoinedInput { nickname: "vanea".to_string() }) }).unwrap();
            tx.send(InputParcel { client_id: vanea_clone.clone().get_id(), input: Input::Posted(PostedInput { content: "cf???".to_string() }) }).unwrap();
            hub.run(rx).await;
        });

       assert_eq!(rx1.recv().await.unwrap(), OutputParcel { client_id: vanea.get_id(), output: Output::Joined(JoinedOutput { nickname: "vanea".to_string() })} );
       assert_eq!(rx1.recv().await.unwrap(), OutputParcel { client_id: vanea.get_id(), output: Output::Posted(PostedOutput { content: "cf???".to_string() })} );

       assert_eq!(rx2.recv().await.unwrap(), OutputParcel { client_id: vanea.get_id(), output: Output::Joined(JoinedOutput { nickname: "vanea".to_string() })} );
       assert_eq!(rx2.recv().await.unwrap(), OutputParcel { client_id: vanea.get_id(), output: Output::Posted(PostedOutput { content: "cf???".to_string() })} );
    }
}
