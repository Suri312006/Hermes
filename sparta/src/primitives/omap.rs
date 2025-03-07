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
