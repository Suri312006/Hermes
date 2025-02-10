use std::collections::HashMap;

//TODO: these arent objivious yet
struct UserData {
    head: u64,
    tail: u64,
}

/*
The user store is implemented as an oblivious map and relates user
identifiers to the head and tail of items in the message store.
*/
pub struct UserStore {
    //TODO: figure out the type for recipient
    // Make this an actual OMAP
    inner: HashMap<u64, UserData>
}

//TODO: implemented as an OMAP
impl UserStore {
    pub fn setup() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn put(&mut self, recipient: u64, user_data: UserData) {
        self.inner.insert(recipient, user_data);
    }

    pub fn get(&self, recipient: u64) -> Option<&UserData> {
        self.inner.get(&recipient)
    }
}
