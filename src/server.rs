mod handler;
mod types;

use handler::MessageHandler;
use tokio_stream::StreamExt;
use tonic::{transport::Server, Request, Response, Status, Streaming};
use tracker_service::tracking_service_server::TrackingService;
use tracker_service::{Event, EventResponse};
use types::*;

pub mod tracker_service {
    // The string specified here must match the proto package name
    tonic::include_proto!("tracker_service");
}

#[derive(Debug, Default)]
pub struct Tracker {}

#[tonic::async_trait]
impl TrackingService for Tracker {
    async fn create_event(
        &self,
        request: Request<Streaming<Event>>,
    ) -> Result<Response<EventResponse>, Status> {
        let mut stream = request.into_inner();

        while let Some(event) = stream.next().await {
            MessageHandler::new(event?).handle().await;
        }

        let reply = EventResponse {
            session_id: String::from("some-id"),
            status: RESPONSE_SUCCESS,
            message: Some(String::from("event received")),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let tracker = Tracker::default();

    println!("Starting server on {:?}", addr);

    Server::builder()
        .add_service(tracker_service::tracking_service_server::TrackingServiceServer::new(tracker))
        .serve(addr)
        .await?;

    Ok(())
}
