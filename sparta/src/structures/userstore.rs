use std::sync::{Arc, Mutex};

use color_eyre::eyre::{Context, Result, eyre};
use ed25519_dalek::PUBLIC_KEY_LENGTH;
use log::debug;
use oram::{
    Address, BlockSize, BlockValue, Oram, OramError, PathOram,
    path_oram::{DEFAULT_BLOCKS_PER_BUCKET, DEFAULT_RECURSION_CUTOFF, DEFAULT_STASH_OVERFLOW_SIZE},
};
use rand::rngs::OsRng;

use crate::{oblivious_select::oblivious_select, rand_address};

use super::messagestore::Recipient;

const DB_SIZE: Address = agora::USER_DB_SIZE;

//TODO: why the fuck is the block size 32
const BLOCK_SIZE: BlockSize = 64;

pub const PUB_KEY_SIZE: usize = 33;

///  Head represents the pointer to the users first message / head
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UserData {
    pub head: u64,
    pub tail: u64,
    pub pub_key: [u8; 33],
}

/*
The user store is implemented as an oblivious map and relates user
identifiers to the head and tail of items in the message store.
*/
#[derive(Debug)]
pub struct UserStoreInner {
    //TODO: figure out the type for recipient
    // Make this an actual OMAP
    // inner: HashMap<u64, UserData>,
    oram: PathOram<BlockValue<BLOCK_SIZE>, 4, BLOCK_SIZE>,
}

pub type UserStore = Arc<Mutex<UserStoreInner>>;

//TODO: implemented as an OMAP
impl UserStoreInner {
    pub fn new() -> Result<Self> {
        let mut rng = rand::rngs::OsRng;

        let path_oram =
            PathOram::<BlockValue<BLOCK_SIZE>, DEFAULT_BLOCKS_PER_BUCKET, BLOCK_SIZE>::new_with_parameters(
                DB_SIZE,
                &mut rng,
                DEFAULT_STASH_OVERFLOW_SIZE,
                DEFAULT_RECURSION_CUTOFF,
            )
            .with_context(|| "Failure when trying to initalize PathORAM for Message Store.")?;

        Ok(Self { oram: path_oram })
    }

    pub fn update_data(
        &mut self,
        recipient: u64,
        user_data: UserData,
        write: bool,
    ) -> Result<Option<UserData>, OramError> {
        debug!("Updating data for user: {recipient}");
        let mut rng = OsRng;
        let new_kv = KeyVal {
            recipient,
            user_data,
            exists: 1,
        };

        let mut ret: KeyVal = [0_u8; BLOCK_SIZE].into();

        for addr in 0..DB_SIZE {
            let block_val: KeyVal = self
                .oram
                .access(
                    addr,
                    |block| {
                        // inside this closure we detemine if we want to update data
                        let kv: KeyVal = block.data.into();

                        let condition = kv.recipient == recipient && write;

                        let ptr_a = &raw const new_kv as u64;
                        let ptr_b = &raw const kv as u64;

                        let final_ptr = oblivious_select(condition, ptr_a, ptr_b);

                        let final_res = final_ptr as *const KeyVal;

                        unsafe {
                            let block = *final_res;
                            BlockValue { data: block.into() }
                        }
                    },
                    &mut rng,
                )?
                .data
                .into();

            // we set ret to block val if recipient of block val and new_kv are the same
            let cond = block_val.recipient == recipient;

            let ptr_a = &raw const block_val as u64;
            let ptr_b = &raw const ret as u64;

            let final_ptr = oblivious_select(cond, ptr_a, ptr_b);

            let final_res = final_ptr as *const KeyVal;
            unsafe { ret = *final_res }
        }

        // we leak whether we found user or not, which is fine
        if ret == [0_u8; BLOCK_SIZE].into() {
            return Ok(None);
        }

        Ok(Some(ret.user_data))
    }

    //TODO: make this a proper error return type
    pub fn add_user(&mut self, recipient: u64, pub_key: &[u8]) -> Result<()> {
        // okay iterate through each entry
        let mut rng = OsRng;

        debug!("Adding user: {recipient}");

        let mut chosen = 0;

        for addr in 0..DB_SIZE {
            let v = self.oram.read(addr, &mut rng)?;

            let block: KeyVal = v.data.into();

            let condition = (block.exists == 0 && chosen == 0) || (recipient == block.recipient);

            chosen = oblivious_select(condition, addr, chosen);
        }

        let head = rand_address();

        if pub_key.len() != PUB_KEY_SIZE {
            return Err(eyre!("invalid pub_key"));
        }

        let mut pub_key_buf = [0_u8; PUB_KEY_SIZE];
        pub_key_buf.copy_from_slice(pub_key);

        // debug!("created user head address: {:?}", head);

        let kv = KeyVal {
            recipient,
            user_data: UserData {
                head,
                tail: head,
                pub_key: pub_key_buf,
            },
            exists: 1,
        };
        debug!("written kv: {:?}", kv);

        let new_block: [u8; BLOCK_SIZE] = kv.into();

        self.oram
            .write(chosen, BlockValue { data: new_block }, &mut rng)?;

        Ok(())
    }
}

impl UserData {
    pub fn new(head: u64, tail: u64, pub_key: [u8; PUB_KEY_SIZE]) -> Self {
        Self {
            head,
            tail,
            pub_key,
        }
    }
}

// sizeof 24 bytes
#[derive(Clone, Copy, Debug, PartialEq)]
struct KeyVal {
    recipient: u64,
    user_data: UserData,

    exists: u8,
}

impl From<KeyVal> for [u8; BLOCK_SIZE] {
    fn from(val: KeyVal) -> Self {
        let mut buf = [0_u8; BLOCK_SIZE];
        buf[0..8].copy_from_slice(&val.recipient.to_le_bytes());
        buf[8..16].copy_from_slice(&val.user_data.head.to_le_bytes());
        buf[16..24].copy_from_slice(&val.user_data.tail.to_le_bytes());

        buf[24..57].copy_from_slice(&val.user_data.pub_key);

        buf[63] = val.exists;

        buf
    }
}
impl From<[u8; BLOCK_SIZE]> for KeyVal {
    fn from(value: [u8; BLOCK_SIZE]) -> Self {
        let recip: Recipient = u64::from_le_bytes(value[0..8].try_into().unwrap());
        let head: Address = u64::from_le_bytes(value[8..16].try_into().unwrap());
        let tail: Address = u64::from_le_bytes(value[16..24].try_into().unwrap());

        let mut pub_key = [0_u8; PUB_KEY_SIZE];

        pub_key.copy_from_slice(&value[24..57]);

        Self {
            recipient: recip,
            user_data: UserData {
                head,
                tail,
                pub_key,
            },
            exists: value[63],
        }
    }
}
