use std::sync::Arc;

use color_eyre::eyre::Result;
use log::{debug, error};
use rand::{thread_rng, Rng};
use tonic::{async_trait, Request, Response, Status};

use crate::{
    messagestore::MessageStore, user_service_server::UserService, userstore::UserStore, NewUserReq,
    NewUserRes,
};

pub struct UserServer {
    user_store: UserStore,
}

impl UserServer {
    pub fn new(user_store: &UserStore) -> Self {
        UserServer {
            user_store: user_store.clone(),
        }
    }
}

#[async_trait]
impl UserService for UserServer {
    async fn create_user(
        self: Arc<Self>,
        req: Request<NewUserReq>,
    ) -> Result<Response<NewUserRes>, Status> {
        // from my understanding, this would have to be inserted into the map
        let mut user_store = self.user_store.lock().map_err(|e| {
            error!("Failed to acquire UserStore lock!");
            Status::new(tonic::Code::Internal, "Internal Error")
        })?;

        let user_id: u32 = thread_rng().gen();

        // TODO: need to make sure this isnt a used up spot
        user_store.add_user(user_id as u64).map_err(|e| {
            error!("{:?}", e);

            Status::new(tonic::Code::Internal, "Internal Error")
        })?;

        // debug!("{:#?}", user_store);

        Ok(Response::new(NewUserRes {
            id: user_id.to_string(),
        }))
    }
}
