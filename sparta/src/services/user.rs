use std::sync::Arc;

use color_eyre::eyre::Result;
use tonic::{async_trait, Request, Response, Status};

use crate::{
    messagestore::MessageStore, user_service_server::UserService, userstore::UserStore, NewUserReq,
    NewUserRes,
};

pub struct UserServer {
    user_store: UserStore,
    message_store: MessageStore,
}

impl UserServer {
    pub fn new(user_store: UserStore, message_store: MessageStore) -> Self {
        UserServer {
            user_store,
            message_store,
        }
    }
}

#[async_trait]
impl UserService for UserServer {
    async fn create_user(
        self: Arc<Self>,
        req: Request<NewUserReq>,
    ) -> Result<Response<NewUserRes>, Status> {
        unimplemented!()
    }
}
