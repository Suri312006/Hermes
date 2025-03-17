use std::{
    fs::canonicalize, path::PathBuf, process::Command, str::FromStr, thread::sleep, time::Duration,
};

use criterion::{Criterion, criterion_group, criterion_main};
use tokio::runtime::Builder;

use agora::{MSG_SIZE, SPARTA_PORT};
use grpc::{
    FetchReq, NewUserReq, Packet, message_service_client::MessageServiceClient,
    user_service_client::UserServiceClient,
};
use tonic::{IntoRequest, transport::Channel};

mod grpc {
    tonic::include_proto!("hermes");
}

const WAIT_TIME: u64 = 8;

async fn fetch_k(mut msg_client: MessageServiceClient<Channel>, user_id: String, k: i32) {
    let resp = msg_client
        .fetch(FetchReq {
            recipient: user_id,
            amount: k,
            sig: String::new(),
        })
        .await
        .unwrap();
}

async fn send(mut msg_client: MessageServiceClient<Channel>, user_id: String, body: Vec<u8>) {
    let resp = msg_client
        .send(Packet {
            recipient: user_id,
            body,
        })
        .await
        .unwrap();
}
fn fetch_benches(c: &mut Criterion) {
    let mut msg_client = None;
    let mut user = None;

    let mut runtime = Builder::new_current_thread().enable_all().build().unwrap();

    let mut handle = Command::new("cargo")
        .args(["run", "--release"])
        .current_dir(canonicalize(PathBuf::from_str("../sparta").unwrap()).unwrap())
        .spawn()
        .expect("Sparta failed to start!");

    // wait for sparta to be up and availible
    sleep(Duration::from_secs(WAIT_TIME));

    runtime.block_on(async {
        let server_url = format!("http://{}", SPARTA_PORT);

        let mut user_client = UserServiceClient::connect(server_url.clone())
            .await
            .expect("Sparta must be running for this benchmark to operate.");
        let mc = MessageServiceClient::connect(server_url.clone())
            .await
            .expect("Sparta must be running for this benchmark to operate.");
        msg_client = Some(mc.clone());

        let user_1 = user_client
            .create_user(NewUserReq {}.into_request())
            .await
            .unwrap()
            .into_inner();
        user = Some(user_1.id);
    });

    let msg_client = &mut msg_client.unwrap();
    let user = user.unwrap();

    let mut f = c.benchmark_group("Fetch");

    f.bench_function("K = 1", |b| {
        b.to_async(&runtime)
            .iter(async || fetch_k(msg_client.clone(), user.clone(), 1).await);
    });

    f.bench_function("K = 10", |b| {
        b.to_async(&runtime)
            .iter(async || fetch_k(msg_client.clone(), user.clone(), 10).await);
    });

    f.bench_function("K = 100", |b| {
        b.to_async(&runtime)
            .iter(async || fetch_k(msg_client.clone(), user.clone(), 100).await);
    });

    f.bench_function("K = 1000", |b| {
        b.to_async(&runtime)
            .iter(async || fetch_k(msg_client.clone(), user.clone(), 1000).await);
    });

    let _ = handle.try_wait();
    handle.kill().unwrap();
}

fn send_benches(c: &mut Criterion) {
    let mut msg_client = None;
    let mut user = None;

    let runtime = Builder::new_current_thread().enable_all().build().unwrap();

    let mut handle = Command::new("cargo")
        .args(["run", "--release"])
        .current_dir(canonicalize(PathBuf::from_str("../sparta").unwrap()).unwrap())
        .spawn()
        .expect("Sparta failed to start!");

    // wait for sparta to be up and availible
    sleep(Duration::from_secs(WAIT_TIME));

    runtime.block_on(async {
        let server_url = format!("http://{}", SPARTA_PORT);

        let mut user_client = UserServiceClient::connect(server_url.clone())
            .await
            .expect("Sparta must be running for this benchmark to operate.");
        let mc = MessageServiceClient::connect(server_url.clone())
            .await
            .expect("Sparta must be running for this benchmark to operate.");
        msg_client = Some(mc.clone());

        let user_1 = user_client
            .create_user(NewUserReq {}.into_request())
            .await
            .unwrap()
            .into_inner();
        user = Some(user_1.id);
    });

    let mut s = c.benchmark_group("Send");

    let msg_client = &mut msg_client.unwrap();
    let user = user.unwrap();

    let mut message = Vec::from("MESSAGE");
    message.resize(MSG_SIZE, 0);

    s.bench_function("K = 1", |b| {
        b.to_async(&runtime).iter(async || {
            send(msg_client.clone(), user.clone(), message.clone()).await;
        });
    });

    let _ = handle.try_wait();
    handle.kill().unwrap();
}

async fn create_user(mut user_client: UserServiceClient<Channel>) {
    let _ = user_client
        .create_user(NewUserReq {}.into_request())
        .await
        .unwrap()
        .into_inner();
}

fn user_benches(c: &mut Criterion) {
    let mut user_client = None;

    let mut handle = Command::new("cargo")
        .args(["run", "--release"])
        .current_dir(canonicalize(PathBuf::from_str("../sparta").unwrap()).unwrap())
        .spawn()
        .expect("Sparta failed to start!");

    // wait for sparta to be up and availible
    sleep(Duration::from_secs(WAIT_TIME));

    let runtime = Builder::new_current_thread().enable_all().build().unwrap();

    runtime.block_on(async {
        let server_url = format!("http://{}", SPARTA_PORT);

        let uc = UserServiceClient::connect(server_url.clone())
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
    let _ = handle.try_wait();
    handle.kill().unwrap();
}

criterion_group!(benches, fetch_benches, send_benches, user_benches);
criterion_main!(benches);
