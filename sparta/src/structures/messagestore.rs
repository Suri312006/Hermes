use color_eyre::eyre::{Context, Result};
use oram::{Address, BlockSize, BlockValue, DefaultOram};

const BLOCK_SIZE: BlockSize = 64;
const DB_SIZE: Address = 64;

/**
    The Message Store is implemented as an ORAM and stores queue nodes,
    with each node storinga message and a pointer to the next node in the
    ORAM


**/
pub struct MessageStore {
    inner: DefaultOram<BlockValue<64>>,
}

//TODO: implemented as an ORAM
impl MessageStore {
    pub fn setup() -> Result<Self> {
        let mut rng = rand::rngs::OsRng;

        let oram = DefaultOram::<BlockValue<BLOCK_SIZE>>::new(DB_SIZE, &mut rng)
            .with_context(|| "Failure when trying to initalize ORAM for Message Store.")?;

        Ok(Self { inner: oram })
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
