use color_eyre::eyre::{Context, Result};
use oram::{
    path_oram::{DEFAULT_BLOCKS_PER_BUCKET, DEFAULT_RECURSION_CUTOFF, DEFAULT_STASH_OVERFLOW_SIZE},
    Address, BlockSize, BlockValue, BucketSize, DefaultOram, OramBlock, PathOram, RecursionCutoff,
    StashSize,
};

const BLOCK_SIZE: BlockSize = 64;
const DB_SIZE: Address = 64;
const BUCKET_SIZE: BucketSize = DEFAULT_BLOCKS_PER_BUCKET;

/**
    The Message Store is implemented as an ORAM and stores queue nodes,
    with each node storinga message and a pointer to the next node in the
    ORAM


**/
pub struct MessageStore {
    inner: PathOram<BlockValue<64>, 4, 64>,
}

//TODO: implemented as an ORAM
impl MessageStore {
    pub fn setup() -> Result<Self> {
        let mut rng = rand::rngs::OsRng;

        // let oram = DefaultOram::<BlockValue<BLOCK_SIZE>>::new(DB_SIZE, &mut rng)
        //     .with_context(|| "Failure when trying to initalize ORAM for Message Store.")?;

        let stash_size = StashSize::from(40_u16);

        let path_oram =
            PathOram::<BlockValue<BLOCK_SIZE>, BUCKET_SIZE, BLOCK_SIZE>::new_with_parameters(
                DB_SIZE,
                &mut rng,
                DEFAULT_STASH_OVERFLOW_SIZE,
                DEFAULT_RECURSION_CUTOFF,
            )
            .with_context(|| "Failure when trying to initalize PathORAM for Message Store.")?;

        Ok(Self { inner: path_oram })
    }

    /**
    API as defined by SPARTA paper
    @param operation := {READ | WRITE}
    @param
    **/
    pub fn access() {
        todo!()
    }
}
