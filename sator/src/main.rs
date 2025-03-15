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
use tonic::IntoRequest;

mod grpc {
    tonic::include_proto!("hermes");
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let server_url = format!("http://{}", SPARTA_PORT);

    let mut user_client = UserServiceClient::connect(server_url.clone()).await?;
    let mut msg_client = MessageServiceClient::connect(server_url.clone()).await?;

    let user_1 = user_client
        .create_user(NewUserReq {}.into_request())
        .await?
        .into_inner();

    let user_2 = user_client
        .create_user(NewUserReq {}.into_request())
        .await?
        .into_inner();

    let msg = "what the fuck".to_owned();
    let mut x = Vec::from(msg);

    x.resize(MSG_SIZE, 0);

    let start = Instant::now();
    // let send_handle = spawn(async move {
    // for i in 0..1 {
    let resp = msg_client
        .fetch(FetchReq {
            recipient: user_2.id.to_string(),
            amount: 1,
        })
        .await;

    let end = Instant::now();
    println!("latency for 1 request: {:?}", end.duration_since(start));

    let start = Instant::now();

    // let mut num = 0;
    // while start.elapsed() < Duration::from_secs(1) {
    let resp = msg_client
        .fetch(FetchReq {
            recipient: user_2.id.to_string(),
            amount: 333,
        })
        .await;
    let end = Instant::now();
    println!("num fetched: 333, time: {:?}", end.duration_since(start));

    // println!("sending 100 requests = {:?}", end.duration_since(start));

    // let resp = msg_client
    //     .send(Packet {
    //         recipient: user_2.id.to_string(),
    //         body: x,
    //     })
    //     .await?;

    Ok(())

    // ideally each user would have a variable request rate right?

    // we want to seed users into the data base
    //
    // we want to have each user ram the server with send messages?
    //
    // then we want to measure throughput of the server
}
