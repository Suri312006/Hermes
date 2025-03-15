// pub(crate) mod avl;
pub(crate) mod oblivious_select;

use rand::{thread_rng, Rng};

use crate::messagestore::DB_SIZE;

#[inline(always)]
pub fn rand_address() -> u64 {
    thread_rng().gen_range(0..DB_SIZE)
}
