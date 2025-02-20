use rand::{random, thread_rng, Rng};

use crate::messagestore::DB_SIZE;

pub mod oblivious_select;
pub mod omap;

#[inline(always)]
pub fn rand_address() -> u64 {
    thread_rng().gen_range(0..DB_SIZE)
}
