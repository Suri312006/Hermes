use color_eyre::eyre::Result;
use log::{debug, error, warn};
use oram::Address;
use rand::RngCore;
use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};

use crate::{
    grpc::{message_service_server::MessageService, Ack, FetchReq, Packet, PacketList},
    primitives::oblivious_select::oblivious_select,
    rand_address,
    structures::{
        messagestore::{MessageNode, MessageStore, Recipient, MESSAGE_SIZE},
        userstore::{UserData, UserStore},
    },
};

pub struct MessageServer {
    user_store: UserStore,
    message_store: MessageStore,
}

impl MessageServer {
    pub fn new(user_store: &UserStore, message_store: &MessageStore) -> Self {
        MessageServer {
            user_store: user_store.clone(),
            message_store: message_store.clone(),
        }
    }
}

#[async_trait]
impl MessageService for MessageServer {
    ///on a send, the location of the tail of the recipient is looked up in
    ///the user store and the message is written at that location in the message store
    async fn send(self: Arc<Self>, req: Request<Packet>) -> Result<Response<Ack>, Status> {
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

        let nexttail = rand_address();

        let recipient: Recipient = req.recipient.parse().map_err(|_| {
            error!("Unable to parse recipient.");
            Status::new(tonic::Code::Internal, "Internal Error")
        })?;

        let Some((body, _)) = req.body.as_slice().split_at_checked(MESSAGE_SIZE) else {
            error!(
                "Couldnt Split Message, len: {:?}",
                req.body.as_slice().len()
            );

            return Err(Status::invalid_argument("Bad Message Body"));
        };

        let mut message: [u8; MESSAGE_SIZE] = [0; MESSAGE_SIZE];

        message.copy_from_slice(body);

        let recipient_data = {
            let mut user_store = self.user_store.lock().map_err(|e| {
                error!("{e}");
                Status::internal("Internal Error.")
            })?;

            let prev_data = user_store
                .get_data(recipient)
                .map_err(|e| {
                    error!("{e}");
                    Status::internal("Internal Error.")
                })?
                .ok_or_else(|| {
                    warn!("User not Found: {recipient}");
                    Status::not_found("User not found")
                })?;

            user_store
                .update_data(recipient, UserData::new(prev_data.head, nexttail))
                .map_err(|e| {
                    error!("{e}");
                    Status::internal("Internal Server Error.")
                })?;

            prev_data
        };

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

    ///On a fetch, the user looks up the head of their queue in the message store,
    ///then follows the pointer in each message node to the next message node.
    async fn fetch(
        self: Arc<Self>,
        req: Request<FetchReq>,
    ) -> Result<Response<PacketList>, Status> {
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

        let recipient: Recipient = req.recipient.parse().map_err(|_| {
            error!("unable to parse recipient");
            Status::internal("Internal Error")
        })?;

        let user_data = {
            let mut user_store = self.user_store.lock().map_err(|_e| {
                error!("Failed to accquire user_store lock");
                Status::internal("Internal Error.")
            })?;

            user_store
                .get_data(recipient)
                .map_err(|e| {
                    error!("{:?}", e);
                    Status::internal("Internal Error")
                })?
                .ok_or_else(|| Status::not_found("Recipient not found."))?
        };

        debug!("real_user_data: {:#?}", user_data);

        let mut messages: Vec<Packet> = Vec::new();

        let mut x = user_data.head;

        while messages.len() < req.amount as usize {
            // might be able to put this outside?
            let dummy: Address = rand_address();

            let mut dummy_msg: [u8; MESSAGE_SIZE] = [0; MESSAGE_SIZE];

            rand::thread_rng().fill_bytes(&mut dummy_msg);

            let dummy_result: MessageNode =
                MessageNode::new(dummy_msg, recipient, dummy, rand_address());

            let condition = x != user_data.tail;

            let access_addr = oblivious_select(condition, x, dummy);

            let oram_result = {
                let mut message_store = self.message_store.lock().map_err(|_| {
                    error!("Failed to accquire message_store lock");
                    Status::internal("Internal Error.")
                })?;

                debug!("ABOUT to read for real result: {:?}", access_addr);
                message_store.read(access_addr).ok_or_else(|| {
                    error!("Failed to access addr");
                    Status::internal("Internal Error.")
                })?
            };

            debug!("Real Result: {:?}", oram_result);
            debug!("Dummy Result: {:?}", dummy_result);

            let final_ptr = oblivious_select(
                condition,
                &raw const oram_result as u64,
                &raw const dummy_result as u64,
            );

            let final_result: *const MessageNode = final_ptr as *const MessageNode;
            // let final_result: *const MessageNode = unsafe { std::mem::transmute(final_ptr) };

            let final_message = unsafe { *final_result };

            x = oblivious_select(condition, oram_result.next, x);

            // debug!("{:?}", final_message);
            messages.push(final_message.into());
        }

        Ok(Response::new(PacketList { inner: messages }))
    }
}
