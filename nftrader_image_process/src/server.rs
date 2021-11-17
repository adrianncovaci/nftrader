#[derive(Debug)]
struct ImageGuidProcess;

pub mod imageprocess {
    tonic::include_proto!("imageprocess");
}

use blake2::{Blake2b, Digest};
use imageprocess::image_guid_server::{ImageGuid, ImageGuidServer};
use imageprocess::Image;
use nftrader_image_process::database::utils::establish_connection;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl ImageGuid for ImageGuidProcess {
    async fn get_image(&self, _request: Request<Image>) -> Result<Response<Image>, Status> {
        let connection = establish_connection();
        println!("aici");
        if let Some(image) = nftrader_image_process::database::imagequeries::get_image(
            &connection,
            _request.get_ref().base64.as_str(),
        ) {
            let response_image = Image {
                base64: image.hashed_value,
            };
            return Ok(Response::new(response_image));
        } else {
            println!("here");
            let mut hasher = Blake2b::new();
            let data = _request.get_ref().base64.as_bytes().to_vec();
            hasher.input(data);
            let hashed = format!("{:?}", hasher.result());
            println!("{}", hashed);
            let hashed = hashed.as_str();
            let new_image =
                nftrader_image_process::database::imagemutations::create_image(&connection, hashed);
            let response_image = Image {
                base64: new_image.hashed_value,
            };
            return Ok(Response::new(response_image));
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();

    let image_process = ImageGuidProcess;

    let svc = ImageGuidServer::new(image_process);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
