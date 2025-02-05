use std::sync::Arc;

use tonic::{async_trait, Request, Response, Status};

use crate::grpc::{message_service_server::MessageService, Ack, FetchReq, Message, MessageList};

pub struct MessageServer {}

impl MessageServer {
    pub fn new() -> Self {
        MessageServer {}
    }
}

#[async_trait]
impl MessageService for MessageServer {
    async fn send(self: Arc<Self>, req: Request<Message>) -> Result<Response<Ack>, Status> {
        Ok(Response::new(Ack {}))
    }

    async fn fetch(
        self: Arc<Self>,
        req: Request<FetchReq>,
    ) -> Result<Response<MessageList>, Status> {
        todo!()
    }
}
