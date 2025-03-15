use std::{
    fs::canonicalize,
    path::PathBuf,
    process::Command,
    str::FromStr,
    thread::sleep,
    time::{self, Duration},
};

use criterion::{Criterion, criterion_group, criterion_main};
use tokio::runtime::Builder;

use agora::SPARTA_PORT;
use grpc::{
    FetchReq, NewUserReq, message_service_client::MessageServiceClient,
    user_service_client::UserServiceClient,
};
use tonic::{IntoRequest, transport::Channel};

mod grpc {
    tonic::include_proto!("hermes");
}

async fn my_async_function(mut msg_client: MessageServiceClient<Channel>, user_id: String) {
    let resp = msg_client
        .fetch(FetchReq {
            recipient: user_id,
            amount: 1,
        })
        .await;
}

fn async_bench(c: &mut Criterion) {
    let mut msg_client = None;
    let mut user = None;

    let mut runtime = Builder::new_current_thread().enable_all().build().unwrap();

    let mut handle = Command::new("cargo")
        .args(["run", "--release"])
        .current_dir(canonicalize(PathBuf::from_str("../sparta").unwrap()).unwrap())
        .spawn()
        .expect("Sparta failed to start!");

    sleep(Duration::from_secs(5));

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

    c.bench_function("my_async_function", |b| {
        b.to_async(&runtime)
            .iter(async || my_async_function(msg_client.clone(), user.clone()).await);
    });

    handle.kill().unwrap();
}

criterion_group!(benches, async_bench);
criterion_main!(benches);
