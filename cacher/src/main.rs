#[macro_use]
extern crate lazy_static;
use std::{collections::HashMap, convert::Infallible};

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use warp::{hyper::Response, Filter};

lazy_static! {
    static ref IMG_DB: RwLock<HashMap<String, String>> = {
        let m = HashMap::new();
        RwLock::new(m)
    };
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    pub id: i32,
    pub hashed_value: String,
}

async fn get_image(content: String) -> Result<String, Infallible> {
    if !IMG_DB.read().await.contains_key(&content) {
        println!("OMG ITS EMPTY");
        return Ok("".into());
    } else {
        println!("DERP");
        let hashed = IMG_DB.read().await.get(&content).unwrap().clone();
        return Ok(hashed);
    }
}

#[tokio::main]
async fn main() {
    let routes = warp::path("cache").and(warp::query()).and_then(
        move |params: HashMap<String, String>| async move {
            match params.get("img") {
                Some(key) => Response::builder()
                    .body(get_image(key.clone()).await.unwrap())
                    .and_then(|resp| Ok(resp))
                    .or_else(|_| Err(warp::reject::reject())),
                None => unreachable!(),
            }
        },
    );
    warp::serve(routes).run(([127, 0, 0, 1], 5060)).await;
}
