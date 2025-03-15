use criterion::{Criterion, criterion_group, criterion_main};
use tokio::runtime::Builder;

use std::time::Duration;

use agora::{MSG_SIZE, SPARTA_PORT};
use color_eyre::eyre::Result;
use grpc::{
    FetchReq, NewUserReq, Packet,
    message_service_client::MessageServiceClient,
    user_service_client::{self, UserServiceClient},
};
use prost::Message;
use tokio::{
    join, spawn,
    time::{Instant, Timeout, timeout},
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

    runtime.block_on(async {
        let server_url = format!("http://{}", SPARTA_PORT);

        let mut user_client = UserServiceClient::connect(server_url.clone())
            .await
            .expect("Sparta must be running for this benchmark to operate.");
        let mut mc = MessageServiceClient::connect(server_url.clone())
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
    // let mut user = &mut user.unwrap();
    let user = user.unwrap();

    c.bench_function("my_async_function", |b| {
        b.to_async(&runtime)
            .iter(async || my_async_function(msg_client.clone(), user.clone()).await);
    });
}

criterion_group!(benches, async_bench);
criterion_main!(benches);
