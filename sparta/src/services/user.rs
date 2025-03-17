use std::sync::Arc;

use bincode::config::standard;
use color_eyre::eyre::Result;
use ed25519_dalek::{VerifyingKey, pkcs8};
use log::{error, info};
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

        let pub_key: Vec<u8> = req.into_inner().public_key;

        let (verifying_key, i): (VerifyingKey, usize) =
            bincode::serde::decode_from_slice(pub_key.as_slice(), standard())
                .map_err(|_| Status::internal("something went wrong"))?;

        use ed25519_dalek::pkcs8::EncodePublicKey;
        let x = verifying_key
            .to_public_key_pem(pkcs8::spki::der::pem::LineEnding::LF)
            .map_err(|_| Status::internal("lol"))?;

        info!("public_key: \n{x}");

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
