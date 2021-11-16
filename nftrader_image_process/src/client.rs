pub mod imageprocess {
    tonic::include_proto!("imageprocess");
}

use std::fs;

use imageprocess::image_guid_client::ImageGuidClient;
use imageprocess::Image;
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ImageGuidClient::connect("http://[::1]:10000").await?;

    let response = client
        .get_image(Request::new(Image {
            base64: fs::read_to_string("base64.txt")?.parse()?,
        }))
        .await?;

    println!("RESPONSE = {:?}", response);
    Ok(())
}
