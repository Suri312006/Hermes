use grpc::{NewUserReq, user_service_client::UserServiceClient};
// use hello_world::{HelloRequest, greeter_client::GreeterClient};
use hyper_util::rt::TokioIo;
use tokio_vsock::{VsockAddr, VsockStream};
use tonic::transport::{Endpoint, Uri};

use ed25519_dalek::pkcs8;
use ed25519_dalek::pkcs8::EncodePrivateKey;
use ed25519_dalek::pkcs8::EncodePublicKey;
use rand_core::OsRng;
use tonic::IntoRequest;

use bincode::serde::encode_to_vec;

pub mod grpc {
    tonic::include_proto!("hermes");
}

#[tokio::main]
async fn main() {
    let endpoint = Endpoint::from_static("http://localhost")
        .connect_with_connector(tower::service_fn(|_: Uri| async {
            Ok::<_, std::io::Error>(TokioIo::new(
                VsockStream::connect(VsockAddr::new(16, 50051)).await?,
            ))
        }))
        .await
        .unwrap();
    let mut user_client = UserServiceClient::new(endpoint);

    let mut rng = OsRng;

    let signing_key = ed25519_dalek::SigningKey::generate(&mut rng);
    let verifying_key = signing_key.verifying_key();

    let encoded_key = encode_to_vec(verifying_key, bincode::config::standard()).unwrap();

    let user_id = user_client
        .create_user(
            NewUserReq {
                public_key: encoded_key,
            }
            .into_request(),
        )
        .await
        .unwrap()
        .into_inner()
        .id;

    let out = signing_key
        .to_pkcs8_pem(pkcs8::spki::der::pem::LineEnding::LF)
        .unwrap();

    let x = verifying_key
        .to_public_key_pem(pkcs8::spki::der::pem::LineEnding::LF)
        .unwrap();

    println!("public key:\n{x}");

    println!("user_id: {}\nprivate key: \n{}", user_id, out.as_str());

    dbg!(user_id);
}
