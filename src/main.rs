use std::sync::Arc;

use nftrader::chat::{hub::Hub, server::Server};

#[tokio::main]
async fn main() {
    let server = Box::leak(Box::new(Server::new(3030, Arc::new(Hub::new(None)))));
    server.run().await;
}
