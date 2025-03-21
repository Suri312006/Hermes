/// Upper Limit for the amount of messages sparta can store.
pub const MSG_DB_SIZE: u64 = 2_u64.pow(19);

/// Upper Limit for the amount of users sparta can store.
pub const USER_DB_SIZE: u64 = 2_u64.pow(6);

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

/// The port that sparta will bind to.
pub const TROJAN_BIND_ADDR: &str = "0.0.0.0";

pub const TROJAN_PORT: &str = "50051";
pub const TROJAN_IP: &str = "13.52.102.59";

pub const SPARTA_VSOCK_CID: u32 = 16;
pub const SPARTA_VSOCK_PORT: u32 = 50051;

pub const VERIFY_PHRASE: &str = "HERMES";

/// The port that the proxy will bind to.
pub const PROXY_PORT: &str = "[::1]:50052";

mod log;
pub use log::*;
