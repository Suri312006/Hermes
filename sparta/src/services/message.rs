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
    /**
    on a send, the location of the tail of the recipient is looked up in
    the user store and the message is written at that location in the message store
    **/
    async fn send(self: Arc<Self>, req: Request<Message>) -> Result<Response<Ack>, Status> {
        /*
        In a send, we take in a recipient and a message. We first precompute
        the address for the next send so that this can be stored in the new
        message node. We then make a requst to the user store to get the
        position for the current message and update it with the new tail value.
        Finally, we write back the message with the precomputed address of the
        next message to the message store.


        */
        Ok(Response::new(Ack {}))
    }

    /**
         On a fetch, the user looks up the head of their queue in the message store,
         then follows the pointer in each message node to the next message node.
    **/
    async fn fetch(
        self: Arc<Self>,
        req: Request<FetchReq>,
    ) -> Result<Response<MessageList>, Status> {
        /*
        In a fetch we take in the recipient and the volume of messages K to read.
        We first look up the head of the queue from the user store, then
        iterate k times making an oblivious request to the message store in
        each iteration. As long as we have not reached the end of the message
        queue as denoted by lasat, we continue making real accesses and
        otherwise make dummy requests to the messages store to avoid
        leaking the true number of messages the user has in the message store.

        */
        let req = req.into_inner();

        todo!()
    }
}
