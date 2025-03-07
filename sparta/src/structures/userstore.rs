use std::sync::{Arc, Mutex};

use color_eyre::eyre::{Context, Result};
use log::debug;
use oram::{
    path_oram::{DEFAULT_BLOCKS_PER_BUCKET, DEFAULT_RECURSION_CUTOFF, DEFAULT_STASH_OVERFLOW_SIZE},
    Address, BlockSize, BlockValue, BucketSize, Oram, OramError, PathOram,
};
use rand::rngs::OsRng;

use crate::{oblivious_select::oblivious_select, rand_address};

use super::messagestore::Recipient;

// const DB_SIZE: Address = 2_u64.pow(8);
const DB_SIZE: Address = 2_u64.pow(1);
const BUCKET_SIZE: BucketSize = DEFAULT_BLOCKS_PER_BUCKET;
const BLOCK_SIZE: BlockSize = 32;

///  Head represents the pointer to the users first message / head
#[derive(Copy, Clone, Debug)]
pub struct UserData {
    pub head: u64,
    pub tail: u64,
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
            PathOram::<BlockValue<BLOCK_SIZE>, BUCKET_SIZE, BLOCK_SIZE>::new_with_parameters(
                DB_SIZE,
                &mut rng,
                DEFAULT_STASH_OVERFLOW_SIZE,
                DEFAULT_RECURSION_CUTOFF,
            )
            .with_context(|| "Failure when trying to initalize PathORAM for Message Store.")?;

        Ok(Self { oram: path_oram })
    }

    pub fn update_data(&mut self, recipient: u64, user_data: UserData) {
        let mut rng = OsRng;
        for addr in 0..DB_SIZE {
            let _ = self.oram.access(
                addr,
                |v| {
                    let key_val: KeyVal = v.data.into();
                    let new_block: [u8; BLOCK_SIZE] = KeyVal {
                        recipient,
                        user_data,
                        exists: 1,
                    }
                    .into();
                    let condition = key_val.recipient == recipient;
                    let final_ptr = oblivious_select(
                        condition,
                        &raw const key_val as u64,
                        &raw const new_block as u64,
                    );

                    let final_result = final_ptr as *const KeyVal;

                    unsafe {
                        let block = *final_result;
                        BlockValue { data: block.into() }
                    }
                },
                &mut rng,
            );
        }
    }

    pub fn add_user(&mut self, recipient: u64) -> Result<(), OramError> {
        // okay iterate through each entry
        let mut rng = OsRng;

        let mut chosen = 0;

        for addr in 0..DB_SIZE {
            let v = self.oram.read(addr, &mut rng)?;

            let block: KeyVal = v.data.into();

            let condition = (block.exists == 0 && chosen == 0) || (recipient == block.recipient);

            chosen = oblivious_select(condition, addr, chosen);
        }

        let head = rand_address();

        debug!("created user head address: {:?}", head);

        let new_block: [u8; BLOCK_SIZE] = KeyVal {
            recipient,
            user_data: UserData { head, tail: head },
            exists: 1,
        }
        .into();

        self.oram
            .write(chosen, BlockValue { data: new_block }, &mut rng)?;

        Ok(())
    }

    pub fn get_data(&mut self, recipient: u64) -> Result<Option<UserData>, OramError> {
        let mut rng = OsRng;

        let mut data = None;

        for addr in 0..DB_SIZE {
            let block = self.oram.read(addr, &mut rng)?;
            let kv: Option<KeyVal> = Some(block.data.into());

            debug!("addr: {:?}, block: {:?}", addr, kv);

            let condition = kv.unwrap().recipient == recipient;
            unsafe {
                debug!("condition: {}, kv: {:?}, data: {:?}", condition, kv, data);
                let data_ptr =
                    oblivious_select(condition, &raw const kv as u64, &raw const data as u64)
                        as *const Option<UserData>;

                debug!("chosen: {:?}", *data_ptr);

                data = *data_ptr;
            }
        }

        Ok(data)
    }
}

impl UserData {
    pub fn new(head: u64, tail: u64) -> Self {
        Self { head, tail }
    }
}

// sizeof 24 bytes
#[derive(Clone, Copy, Debug)]
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
        buf[31] = val.exists;

        buf
    }
}
impl From<[u8; BLOCK_SIZE]> for KeyVal {
    fn from(value: [u8; BLOCK_SIZE]) -> Self {
        let mut recip = [0_u8; 8];
        recip.copy_from_slice(&value[0..8]);

        let recip: Recipient = u64::from_le_bytes(value[0..8].try_into().unwrap());

        let head: Address = u64::from_le_bytes(value[8..16].try_into().unwrap());
        let tail: Address = u64::from_le_bytes(value[16..24].try_into().unwrap());

        Self {
            recipient: recip,
            user_data: UserData { head, tail },
            exists: value[31],
        }
    }
}
// use color_eyre::eyre::{Context, Result};
// use oram::{
//     path_oram::{DEFAULT_BLOCKS_PER_BUCKET, DEFAULT_RECURSION_CUTOFF, DEFAULT_STASH_OVERFLOW_SIZE},
//     Address, BlockSize, BlockValue, BucketSize, PathOram,
// };

// pub const DB_SIZE: Address = 2_u64.pow(8);

// // omap should be generic?
// // T is the size that whatever you want to store in the Omap should be, to the nearest power of two
// pub struct LinearOmap<const K: usize, const V: usize> {
//     oram: PathOram<BlockValue< {K + V}>, 4, {K + V>}>,
// }

// // impl<const T: usize> LinearOmap<T> {
// //     pub fn new(size: u64) -> Result<Self> {
// //         let mut rng = rand::rngs::OsRng;

// //         let path_oram =
// //             PathOram::<BlockValue<T>, DEFAULT_BLOCKS_PER_BUCKET, T>::new_with_parameters(
// //                 size,
// //                 &mut rng,
// //                 DEFAULT_STASH_OVERFLOW_SIZE,
// //                 DEFAULT_RECURSION_CUTOFF,
// //             )
// //             .with_context(|| "Failure when trying to initalize PathORAM for Message Store.")?;

// //         Ok(Self { oram: path_oram })
// //     }

// //     pub fn insert(key ,bytes: [u8; T]) {}
// // }
