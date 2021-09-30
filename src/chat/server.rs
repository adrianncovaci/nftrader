use std::sync::Arc;

use futures::{StreamExt, TryStreamExt};
use tokio::sync::mpsc::{self, UnboundedSender};
use warp::Filter;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_stream::wrappers::BroadcastStream;
use crate::models::{client::Client, proto::InputParcel};

use super::hub::Hub;

pub struct Server {
    port: u16,
    hub: Arc<Hub>
}

impl Server {
    pub fn new(port: u16, hub: Arc<Hub>) -> Self {
        Self {
            port,
            hub
        }
    }

    pub async fn run(&'static self) {
        let (tx, rx) = mpsc::unbounded_channel();

        let chat = warp::path("feed")
            .and(warp::any().map(move || self.hub.clone()))
            .and(warp::any().map(move || tx.clone()))
            .and(warp::ws())
            .map(|hub: Arc<Hub>, tx: UnboundedSender<InputParcel>, ws: warp::ws::Ws | {
                ws.on_upgrade(move |ws| async move {
                    tokio::spawn(Self::process_client(hub, tx, ws));
                })
            });
        
        

        let serving = warp::serve(chat).run(([127, 0, 0, 1], self.port));

        let hub_serve = self.hub.run(rx);
        tokio::select! {
            () = serving => {},
            () = hub_serve => {}
        }
    }

    pub async fn process_client(hub: Arc<Hub>, sender: UnboundedSender<InputParcel>, ws: warp::ws::WebSocket) {
        let mut client = Client::new();
        let subscriber = hub.subscribe();
        let (tx, rx) = mpsc::unbounded_channel();
        let (ws_tx, ws_rx) = ws.split();
        println!("splitted this shit");
        let reading = client.read_input(ws_rx)
            .try_for_each(|item| async {
                sender.send(item).unwrap();
                Ok(())
            });
        let rx = UnboundedReceiverStream::new(rx);
        let subscriber = BroadcastStream::new(subscriber);
        tokio::spawn(rx.forward(ws_tx));
        let writing = client.write_input(subscriber)
            .try_for_each(|item| async {
                tx.send(Ok(item)).unwrap();
                Ok(())
            });
        
        if let Err(err) = tokio::select! {
            result = reading => result,
            result = writing => result,
        } {
            println!("Client connection error: {}", err);
        }

        hub.disconnect_user(client.get_id()).await;
        println!("Client {} disconnected", client.get_id());
        
    }

}
