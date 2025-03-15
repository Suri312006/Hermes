use std::sync::Arc;

use color_eyre::eyre::Result;
use log::error;
use rand::{Rng, thread_rng};
use tonic::{Request, Response, Status, async_trait};

use crate::{NewUserReq, NewUserRes, user_service_server::UserService, userstore::UserStore};

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

        let user_id: u32 = thread_rng().r#gen();

        // TODO: need to make sure this isnt a used up spot
        user_store.add_user(user_id as u64).map_err(|e| {
            error!("{:?}", e);

            Status::new(tonic::Code::Internal, "Internal Error")
        })?;

        Ok(Response::new(NewUserRes {
            id: user_id.to_string(),
        }))
    }
}
