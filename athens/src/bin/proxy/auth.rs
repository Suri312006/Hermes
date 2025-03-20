use agora::VERIFY_PHRASE;
use ed25519_dalek::{SIGNATURE_LENGTH, Signature};
use log::STATIC_MAX_LEVEL;
use tonic::{Status, async_trait, body::BoxBody};

use ed25519_dalek::Verifier;

//NOTE: Need to use this specific import
use tonic::codegen::http::{HeaderValue, Request};

use tonic_middleware::RequestInterceptor;

use crate::state::{self, State};

#[derive(Clone)]
pub struct AuthInterceptor {}

impl AuthInterceptor {
    pub fn new() -> Self {
        AuthInterceptor {}
    }
}

#[async_trait]
impl RequestInterceptor for AuthInterceptor {
    async fn intercept(&self, mut req: Request<BoxBody>) -> Result<Request<BoxBody>, Status> {
        match req.headers().get("signature").map(|v| v.to_str()) {
            Some(Ok(sig)) => {
                let sig_bytes =
                    hex::decode(sig).map_err(|e| Status::unauthenticated("invalid signature"))?;

                let mut sig_buf = [0_u8; SIGNATURE_LENGTH];
                if sig_bytes.len() != 64 {
                    return Err(Status::unauthenticated("Bad Signature"));
                }

                sig_buf.copy_from_slice(&sig_bytes.as_slice());

                let sig = Signature::from_bytes(&sig_buf);

                let state = State::read().map_err(|e| Status::internal("dont worry about it"))?;

                // let found = false;
                let mut dev_key = None;
                let mut dev_id = 0;

                for (id, key) in state.device_pub_keys.iter().enumerate() {
                    if key.verify(VERIFY_PHRASE.as_bytes(), &sig).is_ok() {
                        dev_key = Some(key);
                        dev_id = id;
                    }
                }

                if dev_key.is_none() {
                    return Err(Status::unauthenticated("Not an authenticated device"));
                }

                req.headers_mut().insert(
                    "dev_id",
                    HeaderValue::from_str(dev_id.to_string().as_str())
                        .map_err(|_| Status::internal("weird header"))?,
                );
                Ok(req)
            }

            _ => Err(Status::unauthenticated("lock in")),
        }
    }
}
