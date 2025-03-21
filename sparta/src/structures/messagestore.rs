use std::sync::{Arc, Mutex};

use color_eyre::eyre::{Context, Result};
use log::{debug, error};
use oram::{
    path_oram::{DEFAULT_BLOCKS_PER_BUCKET, DEFAULT_RECURSION_CUTOFF, DEFAULT_STASH_OVERFLOW_SIZE},
    Address, BlockSize, BlockValue, Oram, OramError, PathOram,
};
use rand::rngs::OsRng;

use crate::grpc::Packet;

pub const DB_SIZE: Address = agora::MSG_DB_SIZE;
const BLOCK_SIZE: BlockSize = agora::MSG_STORE_BLOCK_SIZE;

pub const MESSAGE_SIZE: usize = agora::MSG_SIZE;

pub type Recipient = u64;

#[derive(Copy, Clone, Debug)]
pub struct MessageNode {
    message: [u8; MESSAGE_SIZE],
    recipient: u64,
    pub curr: Address,
    pub next: Address,
}

/**
    The Message Store is implemented as an ORAM and stores queue nodes,
    with each node storing a message and a pointer to the next node in the
    ORAM
**/
pub struct MessageStoreInner {
    inner: PathOram<BlockValue<BLOCK_SIZE>, 4, BLOCK_SIZE>,
}

pub type MessageStore = Arc<Mutex<MessageStoreInner>>;

impl MessageStoreInner {
    /// in the paper its called setup but in rust we usually use new;
    pub fn new() -> Result<Self> {
        let mut rng = rand::rngs::OsRng;

        // https://www.youtube.com/watch?v=iGfgngtVLr4
        // pathoram talk
        let path_oram =
            PathOram::<BlockValue<BLOCK_SIZE>, DEFAULT_BLOCKS_PER_BUCKET, BLOCK_SIZE>::new_with_parameters(
                DB_SIZE,
                &mut rng,
                DEFAULT_STASH_OVERFLOW_SIZE,
                DEFAULT_RECURSION_CUTOFF,
            )
            .with_context(|| "Failure when trying to initalize PathORAM for Message Store.")?;

        Ok(Self { inner: path_oram })
    }

    pub fn read(&mut self, address: Address) -> Option<MessageNode> {
        let mut rng = OsRng;

        debug!("Reading address: {address}");

        Some(
            self.inner
                .read(address, &mut rng)
                .map_err(|e| {
                    error!("{e}");
                    e
                })
                .ok()?
                .data
                .into(),
        )
    }

    pub fn write(&mut self, msg_node: MessageNode) -> Result<(), OramError> {
        let mut rng = OsRng;

        let curr = msg_node.curr;
        debug!("Writing address: {curr}");

        let data = BlockValue::new(msg_node.into());

        self.inner.write(curr, data, &mut rng).map(|_| ())
    }
}

impl From<[u8; BLOCK_SIZE]> for MessageNode {
    fn from(value: [u8; BLOCK_SIZE]) -> Self {
        let mut message = [0_u8; MESSAGE_SIZE];
        message.copy_from_slice(&value[0..MESSAGE_SIZE]);

        let next: Address =
            u64::from_be_bytes(value[MESSAGE_SIZE..MESSAGE_SIZE + 8].try_into().unwrap());

        let curr: Address = u64::from_be_bytes(
            value[MESSAGE_SIZE + 8..MESSAGE_SIZE + 16]
                .try_into()
                .unwrap(),
        );
        let recipient: Recipient = u64::from_be_bytes(
            value[MESSAGE_SIZE + 16..MESSAGE_SIZE + 24]
                .try_into()
                .unwrap(),
        );

        Self {
            message,
            next,
            curr,
            recipient,
        }
    }
}

impl From<MessageNode> for [u8; BLOCK_SIZE] {
    fn from(val: MessageNode) -> Self {
        let mut buf = [0_u8; BLOCK_SIZE];
        buf[0..MESSAGE_SIZE].copy_from_slice(&val.message);
        buf[MESSAGE_SIZE..MESSAGE_SIZE + 8].copy_from_slice(&val.next.to_be_bytes());
        buf[MESSAGE_SIZE + 8..MESSAGE_SIZE + 16].copy_from_slice(&val.curr.to_be_bytes());
        buf[MESSAGE_SIZE + 16..MESSAGE_SIZE + 24].copy_from_slice(&val.recipient.to_be_bytes());

        buf
    }
}

impl MessageNode {
    pub fn new(
        message: [u8; MESSAGE_SIZE],
        recipient: Recipient,
        curr: Address,
        next: Address,
    ) -> Self {
        Self {
            message,
            recipient,
            curr,
            next,
        }
    }
}

impl From<MessageNode> for Packet {
    fn from(val: MessageNode) -> Self {
        Packet {
            //NOTE: not sure if this is leakage
            recipient: val.recipient.to_string(),
            body: Vec::from_iter(val.message),
        }
    }
}
