use std::{
    fs::canonicalize, path::PathBuf, process::Command, str::FromStr, thread::sleep, time::Duration,
};

use bincode::serde::encode_to_vec;
use criterion::{Criterion, criterion_group, criterion_main};
use ed25519_dalek::ed25519::signature::SignerMut;
use rand_core::OsRng;
use tokio::runtime::Builder;

use agora::{MSG_SIZE, TROJAN_BIND_ADDR, TROJAN_PORT};
use grpc::{
    FetchReq, NewUserReq, Packet, message_service_client::MessageServiceClient,
    trojan_service_client::TrojanServiceClient, user_service_client::UserServiceClient,
};
use tonic::{IntoRequest, transport::Channel};

mod grpc {
    tonic::include_proto!("hermes");
}

const WAIT_TIME: u64 = 8;

async fn fetch_k(
    mut msg_client: TrojanServiceClient<Channel>,
    user_id: String,
    k: i32,
    sig: Vec<u8>,
) {
    let resp = msg_client
        .fetch(FetchReq {
            recipient: user_id,
            amount: k,
            sig,
        })
        .await
        .unwrap();
}

async fn send(mut msg_client: TrojanServiceClient<Channel>, user_id: String, body: Vec<u8>) {
    let resp = msg_client
        .send(Packet {
            recipient: user_id,
            body,
        })
        .await
        .unwrap();
}
fn fetch_benches(c: &mut Criterion) {
    let mut client = None;
    let mut user = None;

    let mut key = None;

    let mut runtime = Builder::new_current_thread().enable_all().build().unwrap();

    runtime.block_on(async {
        let server_url = format!("http://{}:{}", TROJAN_BIND_ADDR, TROJAN_PORT);

        let mut i_client = TrojanServiceClient::connect(server_url.clone())
            .await
            .expect("Sparta must be running for this benchmark to operate.");

        let mut rng = OsRng;
        let signing_key = ed25519_dalek::SigningKey::generate(&mut rng);
        let verifying_key = signing_key.verifying_key();

        let encoded_key = encode_to_vec(verifying_key, bincode::config::standard())
            .expect("should have been able to encode it");

        let user_1 = i_client
            .create_user(
                NewUserReq {
                    public_key: encoded_key,
                }
                .into_request(),
            )
            .await
            .unwrap()
            .into_inner();
        user = Some(user_1.id);
        client = Some(i_client);
        key = Some(signing_key)
    });

    let client = &mut client.unwrap();
    let user = user.unwrap();
    let mut key = key.unwrap();

    let sig = key.sign(user.as_bytes());
    let sig = sig.to_bytes().to_vec();

    let mut f = c.benchmark_group("Fetch");

    f.bench_function("K = 1", |b| {
        b.to_async(&runtime)
            .iter(async || fetch_k(client.clone(), user.clone(), 1, sig.clone()).await);
    });

    f.bench_function("K = 10", |b| {
        b.to_async(&runtime)
            .iter(async || fetch_k(client.clone(), user.clone(), 10, sig.clone()).await);
    });

    f.bench_function("K = 100", |b| {
        b.to_async(&runtime)
            .iter(async || fetch_k(client.clone(), user.clone(), 100, sig.clone()).await);
    });

    f.bench_function("K = 1000", |b| {
        b.to_async(&runtime)
            .iter(async || fetch_k(client.clone(), user.clone(), 1000, sig.clone()).await);
    });
}

fn send_benches(c: &mut Criterion) {
    let mut client = None;
    let mut user = None;
    let mut key = None;

    let runtime = Builder::new_current_thread().enable_all().build().unwrap();

    // wait for sparta to be up and availible
    sleep(Duration::from_secs(WAIT_TIME));

    runtime.block_on(async {
        let server_url = format!("http://{}:{}", TROJAN_BIND_ADDR, TROJAN_PORT);

        let mut i_client = TrojanServiceClient::connect(server_url.clone())
            .await
            .expect("Sparta must be running for this benchmark to operate.");

        let mut rng = OsRng;
        let signing_key = ed25519_dalek::SigningKey::generate(&mut rng);
        let verifying_key = signing_key.verifying_key();

        let encoded_key = encode_to_vec(verifying_key, bincode::config::standard())
            .expect("should have been able to encode it");

        let user_1 = i_client
            .create_user(
                NewUserReq {
                    public_key: encoded_key,
                }
                .into_request(),
            )
            .await
            .unwrap()
            .into_inner();
        user = Some(user_1.id);
        client = Some(i_client);
        key = Some(signing_key)
    });
    let client = &mut client.unwrap();
    let user = user.unwrap();
    let mut key = key.unwrap();

    let sig = key.sign(user.as_bytes());
    let sig = sig.to_bytes().to_vec();

    let mut s = c.benchmark_group("Send");

    let mut message = Vec::from("MESSAGE");
    message.resize(MSG_SIZE, 0);

    s.bench_function("K = 1", |b| {
        b.to_async(&runtime).iter(async || {
            send(client.clone(), user.clone(), message.clone()).await;
        });
    });
}

async fn create_user(mut user_client: TrojanServiceClient<Channel>) {
    let mut rng = OsRng;
    let signing_key = ed25519_dalek::SigningKey::generate(&mut rng);
    let verifying_key = signing_key.verifying_key();

    let encoded_key = encode_to_vec(verifying_key, bincode::config::standard())
        .expect("should have been able to encode it");

    let _ = user_client
        .create_user(
            NewUserReq {
                public_key: encoded_key,
            }
            .into_request(),
        )
        .await
        .unwrap()
        .into_inner();
}

fn user_benches(c: &mut Criterion) {
    let mut user_client = None;

    let runtime = Builder::new_current_thread().enable_all().build().unwrap();

    runtime.block_on(async {
        let server_url = format!("http://{}:{}", TROJAN_BIND_ADDR, TROJAN_PORT);

        let uc = TrojanServiceClient::connect(server_url.clone())
            .await
            .expect("Sparta must be running for this benchmark to operate.");
        user_client = Some(uc);
    });

    let mut u = c.benchmark_group("User");
    let user_client = user_client.expect("User Client should be instantiated");

    u.bench_function("Create User", |b| {
        b.to_async(&runtime).iter(async || {
            create_user(user_client.clone()).await;
        });
    });
}

criterion_group!(benches, fetch_benches, send_benches, user_benches);
criterion_main!(benches);
