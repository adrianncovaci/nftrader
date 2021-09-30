use futures::{Stream, TryStream, TryStreamExt, future, stream::{SplitSink, SplitStream}};
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};
use futures::StreamExt;

use super::{error::Error, proto::{InputParcel, OutputParcel}};
use super::error::Result;

#[derive(Clone)]
pub struct Client {
    client_id: Uuid,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client_id: Uuid::new_v4()
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.client_id
    }

    pub fn read_input(&mut self, stream: SplitStream<WebSocket>) -> impl Stream<Item = Result<InputParcel>> {
        let client_id = self.client_id;
        stream.take_while(|msg| future::ready(
                match msg {
                    Ok(message) => message.is_text(),
                    Err(_) => false,
                }
            ))
            .map(move |msg| {
                match msg {
                    Ok(message) => {
                        let input = serde_json::from_str(message.to_str().unwrap()).unwrap();
                        Ok(InputParcel { client_id, input })
                    },
                    Err(err) => Err(Error::System(err.to_string()))
                }
            })
    }

    pub fn write_input<S>(&mut self, stream: S) -> impl Stream<Item = Result<Message>>
        where
            S: TryStream<Ok = OutputParcel, Error = BroadcastStreamRecvError> + Stream<Item = std::result::Result<OutputParcel, BroadcastStreamRecvError>>  { 
        let client_id = self.client_id;
        stream.try_filter(move |item| {
            future::ready(item.client_id == client_id)
        })
        .map_ok(move |message| {
            let input = serde_json::to_string(&message).unwrap();
            Message::text(input)
        })
        .map_err(|err| {
            Error::System(err.to_string())
        })
    }
}
