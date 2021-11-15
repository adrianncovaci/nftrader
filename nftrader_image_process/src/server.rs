#[derive(Debug)]
struct ImageGuidProcess;

pub mod imageprocess {
    tonic::include_proto!("imageprocess");
}

use imageprocess::image_guid_server::{ImageGuid, ImageGuidServer};
use imageprocess::Image;
use tonic::{Request, Response};

#[tonic::async_trait]
impl ImageGuid for ImageGuidProcess {
    async fn get_image(
        &self,
        _request: Request<Image>,
    ) -> Result<Response<Image>, Box<dyn std::error::Error>> {
    }
}

#[tokio::main]
async fn main() {}
