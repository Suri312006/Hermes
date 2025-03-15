/// Upper Limit for the amount of messages sparta can store.
pub const MSG_DB_SIZE: u64 = 2_u64.pow(17);

/// Upper Limit for the amount of users sparta can store.
pub const USER_DB_SIZE: u64 = 2_u64.pow(8);

/// Oram Block size for the message store
/// NOTE: must be to a power of 2
pub const MSG_STORE_BLOCK_SIZE: usize = 256;

/// The size of an address inside of the message store, in bytes.
pub const MSG_ADDRESS_SIZE: u8 = 8;

/// The size of a recipient inside of the message store, in bytes.
pub const MSG_RECIPIENT_SIZE: u8 = 8;

/// The maximum message size that can be held inside a single message
/// store block.
pub const MSG_SIZE: usize =
    MSG_STORE_BLOCK_SIZE - (2 * MSG_ADDRESS_SIZE) as usize - MSG_RECIPIENT_SIZE as usize;
