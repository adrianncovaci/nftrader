#[macro_use]
extern crate lazy_static;

use anyhow::Result;
use futures::{future, StreamExt, TryStreamExt};
use hyper::Response;
use mongodb::options::ClientOptions;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::Infallible, net::SocketAddr};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::RwLock,
};
use tokio_tungstenite::connect_async;
use warp::{Error, Filter};

lazy_static! {
    static ref USER_DB: RwLock<HashMap<String, User>> = {
        let m = HashMap::new();
        RwLock::new(m)
    };
}

pub mod imageprocess {
    tonic::include_proto!("imageprocess");
}

use std::fs;

use imageprocess::image_guid_client::ImageGuidClient;
use imageprocess::Image;
use tonic::Request;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    client_id: String,
    nickname: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct SelfJoinedOutput {
    pub client_id: String,
    pub nickname: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct JoinedOutput {
    pub nickname: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct OutputParcel {
    pub client_id: String,
    pub output: Output,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "output", content = "payload")]
pub enum Output {
    Posted(PostedOutput),
    SelfJoined(SelfJoinedOutput),
    Joined(JoinedOutput),
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PostedOutput {
    pub content: String,
}

async fn get_db_handle() -> Result<mongodb::Database> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = mongodb::Client::with_options(client_options)?;
    let db = client.database("nftrader_messages");
    return Ok(db);
}

async fn handle_ws_chat() {
    let connect_addr = "127.0.0.1:4040";

    let try_socket = TcpListener::bind(&connect_addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", connect_addr);

    let db_handle = get_db_handle().await.unwrap();

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_ws_stream(stream, addr, db_handle.clone()));
    }
}

async fn insert_user(output: SelfJoinedOutput, db_handle: mongodb::Database) {
    if !USER_DB.read().await.contains_key(&output.client_id) {
        let new_user = User {
            client_id: output.client_id.clone(),
            nickname: output.nickname,
        };
        USER_DB
            .write()
            .await
            .insert(output.client_id, new_user.clone());
        let user_collection = db_handle.collection::<User>("users");
        user_collection.insert_one(new_user, None).await.unwrap();
    }
}

async fn handle_ws_stream(raw_stream: TcpStream, addr: SocketAddr, db_handle: mongodb::Database) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let (outgoing, incoming) = ws_stream.split();

    let (ws_stream, _) = connect_async("ws://127.0.0.1:3030/feed")
        .await
        .expect("Failed to connect");

    let (write, read) = ws_stream.split();

    let proxy_ws = incoming
        .try_filter(|msg| {
            println!("{:?}", msg.to_string());
            future::ready(msg.is_text() || msg.is_binary())
        })
        .forward(write);

    let chat_ws = read
        .try_filter(|msg| {
            if msg.is_text() || msg.is_binary() {
                let output: OutputParcel = serde_json::from_str(&msg.to_string()).unwrap();
                match output.output {
                    Output::SelfJoined(joined_output) => {
                        tokio::spawn(insert_user(joined_output, db_handle.clone()));
                    }
                    Output::Posted(_) => {}
                    Output::Joined(_) => {}
                }
                future::ready(msg.is_text() || msg.is_binary())
            } else {
                future::ready(false)
            }
        })
        .forward(outgoing);

    future::select(proxy_ws, chat_ws).await;
}

async fn process_image(content: String) -> Result<String, Infallible> {
    let mut client = ImageGuidClient::connect("http://[::1]:10000")
        .await
        .unwrap();

    let img = &content[content.len() - 50..content.len()];
    let body = reqwest::Client::new()
        .get("http://127.0.0.1:5060/cache")
        .query(&[("img", img)])
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    if body.is_empty() {
        println!("body = {:?}", body);

        let response = client
            .get_image(Request::new(Image { base64: body }))
            .await
            .unwrap();
        println!("{:?}", response);
        let image = response.get_ref();
        Ok(image.base64.clone())
    } else {
        Ok(body)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut handles = vec![];
    handles.push(tokio::spawn(handle_ws_chat()));

    let routes = warp::path("process").and(warp::query()).and_then(
        move |params: HashMap<String, String>| async move {
            match params.get("img") {
                Some(key) => Response::builder()
                    .body(process_image(key.clone()).await.unwrap())
                    .and_then(|resp| Ok(resp))
                    .or_else(|_| Err(warp::reject::reject())),
                None => unreachable!(),
            }
        },
    );

    warp::serve(routes).run(([127, 0, 0, 1], 5050)).await;
    future::join_all(handles).await;

    Ok(())
}
