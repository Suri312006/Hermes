use std::sync::{Arc, Mutex};

use color_eyre::eyre::Result;
use log::{error, info};
use oram::Address;
use rand::{random, RngCore};
use tonic::{async_trait, Request, Response, Status};

use crate::{
    grpc::{message_service_server::MessageService, Ack, FetchReq, Message, MessageList},
    primitives::oblivious_select::oblivious_select,
    structures::{
        messagestore::{MessageNode, MessageStore, Recipient, MESSAGE_SIZE},
        userstore::UserStore,
    },
};

pub struct MessageServer {
    user_store: Mutex<UserStore>,
    message_store: Mutex<MessageStore>,
}

impl MessageServer {
    pub fn new() -> Result<Self> {
        Ok(MessageServer {
            user_store: Mutex::new(UserStore::setup()),
            message_store: Mutex::new(MessageStore::setup()?),
        })
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

        /*
        nexttail <- U(0, 2^l - 1)
        rand <- U(0, 2^l - 1)
        (head, tail) <- US.update(r, (head, nexttail))
        MS.access(write, rand, (r, tail, nexttail, m))
        */

        let req = req.into_inner();

        let nexttail = random::<u64>();

        let recipient: Recipient = req
            .recipient
            .parse()
            .map_err(|_| Status::new(tonic::Code::Internal, "Internal Error"))?;

        let Some((body, _)) = req.body.as_slice().split_at_checked(MESSAGE_SIZE) else {
            return Err(Status::invalid_argument("Bad Message Body"));
        };

        let mut message: [u8; MESSAGE_SIZE] = [0; MESSAGE_SIZE];

        message.copy_from_slice(body);

        let user_store = self.user_store.lock().map_err(|e| {
            error!("{e}");
            Status::internal("Internal Error.")
        })?;

        let recipient_data = user_store.get(recipient).ok_or_else(|| {
            info!("User not Found: {recipient}");
            Status::not_found("User not found")
        })?;

        self.message_store
            .lock()
            .map_err(|_| Status::internal("Internal Error."))?
            .write(MessageNode::new(
                message,
                recipient,
                recipient_data.tail,
                nexttail,
            ))
            .map_err(|e| {
                error!("{e}");
                Status::internal("Internal Error.")
            })?;

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

        /*
        (first, last) <- US.update(r, (last, last))
        x = first, M = {}
        while |M| < k do
            if x != last then
                (r, curr, next, m) <- MS.access(read, x, NULL)
                x = next
            else
                (_,_,_,m) <- MS.access(read, dummy, NULL)
            endif
            M = M union {m}
        end while
        return M
        */
        let req = req.into_inner();

        let recipient: Recipient = req
            .recipient
            .parse()
            .map_err(|_| Status::new(tonic::Code::Internal, "Internal Error"))?;

        let user_store = self
            .user_store
            .lock()
            .map_err(|_| Status::internal("Internal Error."))?;

        let mut message_store = self
            .message_store
            .lock()
            .map_err(|_| Status::internal("Internal Error."))?;

        let user_data = user_store
            .get(recipient)
            .ok_or_else(|| Status::not_found("Recipient not found."))?;

        let mut messages: Vec<Message> = Vec::new();

        let mut x = user_data.head;

        while messages.len() < req.amount as usize {
            let dummy: Address = random();

            let mut dummy_msg: [u8; MESSAGE_SIZE] = [0; MESSAGE_SIZE];

            rand::thread_rng().fill_bytes(&mut dummy_msg);

            let dummy_result: MessageNode =
                MessageNode::new(dummy_msg, recipient.into(), dummy, random());

            let condition = x != user_data.tail;

            let access_addr = oblivious_select(condition, x, dummy);

            let oram_result = message_store
                .read(access_addr)
                .ok_or_else(|| Status::internal("Internal Error."))?;

            // i have absolutely zero clue if this works lol
            let final_ptr = oblivious_select(
                condition,
                &raw const oram_result as u64,
                &raw const dummy_result as u64,
            );

            let final_result: *const MessageNode = unsafe { std::mem::transmute(final_ptr) };

            let final_message = unsafe { *final_result };

            x = oblivious_select(condition, oram_result.next, x);

            messages.push(final_message.into());
        }

        Ok(Response::new(MessageList { inner: messages }))

        // todo!()
    }
}
