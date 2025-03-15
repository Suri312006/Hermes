use std::time::Duration;

use agora::SPARTA_PORT;
use color_eyre::eyre::Result;
use sparta::{
    message_service_client::MessageServiceClient, user_service_client::UserServiceClient, FetchReq,
    NewUserReq, Packet, Sparta,
};
use tokio::{join, time::timeout};
use tonic::IntoRequest;

#[tokio::test()]
pub async fn simple_send_recv() -> Result<()> {
    let test_duration = Duration::from_secs(10);
    let server = tokio::spawn(async move {
        let sparta = Sparta::new().unwrap();
        timeout(test_duration, sparta.run())
            .await
            // this gets rid of the timeout result
            .unwrap_or_else(|_x| Ok(()))
            .unwrap()
    });
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

    let server_url = format!("http://{}", SPARTA_PORT);
    let mut usr_client = UserServiceClient::connect(server_url.clone())
        .await
        .unwrap();
    let mut msg_client = MessageServiceClient::connect(server_url.clone())
        .await
        .unwrap();

    let recip = usr_client
        .create_user(NewUserReq {}.into_request())
        .await
        .unwrap()
        .into_inner()
        .id;

    let recip2 = usr_client
        .create_user(NewUserReq {}.into_request())
        .await
        .unwrap()
        .into_inner()
        .id;

    let message = "penis penis penis";

    let mut msg_buf: [u8; 232] = [0; 232];

    msg_buf[0] = message.len() as u8;

    msg_buf[1..message.len() + 1].copy_from_slice(message.as_bytes());

    let res = msg_client
        .send(Packet {
            recipient: recip.clone(),
            body: msg_buf.into(),
        })
        .await
        .unwrap()
        .into_inner();

    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

    let res = msg_client
        .fetch(FetchReq {
            recipient: recip.clone(),
            amount: 1,
        })
        .await
        .unwrap()
        .into_inner()
        .inner;

    let first_msg = res.clone().first().unwrap().body.clone();

    let len = *first_msg.first().unwrap() as usize;
    assert_eq!(msg_buf, first_msg.as_slice());

    let recieved_message = String::from_utf8(first_msg[1..len + 1].to_vec()).unwrap();

    assert_eq!(res.first().unwrap().recipient, recip);

    assert_eq!(recieved_message, message);

    let server = join!(server);

    server.0.expect("Server Panic");

    Ok(())
}
