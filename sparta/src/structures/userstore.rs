use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

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
    inner: HashMap<u64, UserData>,
}

pub type UserStore = Arc<Mutex<UserStoreInner>>;

//TODO: implemented as an OMAP
impl UserStoreInner {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn put(&mut self, recipient: u64, user_data: UserData) -> Option<UserData> {
        self.inner.insert(recipient, user_data)
    }

    pub fn get(&self, recipient: u64) -> Option<UserData> {
        self.inner.get(&recipient).copied()
    }
}

impl UserData {
    pub fn new(head: u64, tail: u64) -> Self {
        Self { head, tail }
    }
}
