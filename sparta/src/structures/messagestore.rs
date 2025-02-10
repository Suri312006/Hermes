use core::str;

use color_eyre::{
    eyre::{Context, Result},
    owo_colors::colors::css::MediumAquaMarine,
};
use oram::{
    path_oram::{DEFAULT_BLOCKS_PER_BUCKET, DEFAULT_RECURSION_CUTOFF, DEFAULT_STASH_OVERFLOW_SIZE},
    Address, BlockSize, BlockValue, BucketSize, Oram, OramError, PathOram, StashSize,
};
use rand::rngs::OsRng;

const DB_SIZE: Address = 64;
const BUCKET_SIZE: BucketSize = DEFAULT_BLOCKS_PER_BUCKET;
const BLOCK_SIZE: BlockSize = 1032;

struct MessageNode {
    message: [u8; BLOCK_SIZE - 8],
    next: Address,
}

/**
    The Message Store is implemented as an ORAM and stores queue nodes,
    with each node storing a message and a pointer to the next node in the
    ORAM
**/
pub struct MessageStore {
    inner: PathOram<BlockValue<BLOCK_SIZE>, 4, BLOCK_SIZE>,
}

impl MessageStore {
    pub fn setup() -> Result<Self> {
        let mut rng = rand::rngs::OsRng;

        let stash_size = StashSize::from(40_u16);

        // https://www.youtube.com/watch?v=iGfgngtVLr4
        // pathoram talk
        let mut path_oram =
            PathOram::<BlockValue<BLOCK_SIZE>, BUCKET_SIZE, BLOCK_SIZE>::new_with_parameters(
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

        Some(self.inner.read(address, &mut rng).ok()?.data.into())
    }

    pub fn write(&mut self, address: Address, msg_node: MessageNode) -> Result<(), OramError> {
        let mut rng = OsRng;

        let data = BlockValue::new(msg_node.into());

        self.inner.write(address, data, &mut rng).map(|_| ())
    }
}
impl From<[u8; BLOCK_SIZE]> for MessageNode {
    fn from(value: [u8; BLOCK_SIZE]) -> Self {
        let mut message = [0_u8; BLOCK_SIZE - 8];
        message.copy_from_slice(&value[0..BLOCK_SIZE - 8]);

        let next: Address =
            u64::from_be_bytes(value[BLOCK_SIZE - 8..BLOCK_SIZE].try_into().unwrap());

        Self { message, next }
    }
}

impl Into<[u8; BLOCK_SIZE]> for MessageNode {
    fn into(self) -> [u8; BLOCK_SIZE] {
        let mut buf = [0_u8; BLOCK_SIZE];
        buf[0..BLOCK_SIZE - 8].copy_from_slice(&self.message);
        buf[BLOCK_SIZE - 8..BLOCK_SIZE].copy_from_slice(&self.next.to_be_bytes());

        buf
    }
}
