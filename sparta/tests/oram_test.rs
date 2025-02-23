use std::time::Duration;

use color_eyre::eyre::Result;
use sparta::{message_service_client::MessageServiceClient, FetchReq, Sparta};
use tokio::{join, time::timeout};

// #[tokio::test()]
pub async fn oram_default_test() -> Result<()> {
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

    let soc = "http://[::1]:50051";

    let recipient = "12345678".to_owned();

    let mut client = MessageServiceClient::connect(soc).await.unwrap();
    let res = client
        .fetch(FetchReq {
            recipient,
            amount: 10,
        })
        .await
        .unwrap();

    for e in res.into_inner().inner {
        println!("{:?}", e);
    }

    let server = join!(server);
    server.0.expect("Server Panic");
    Ok(())

    // let res = res.into_inner();
    // for e in res.me
    //     let (db, connection_string) = common::TestDb::connect().await?;

    //     let conf = Config {
    //         db_url: connection_string,
    //         auth_secret: "lmfao-rand-auth-thing".to_string(),
    //     };

    //     let test_duration = Duration::from_secs(10);

    //     let message_count = 1_000;

    //     let server = tokio::spawn(async move {

    //         let hermes = Hermes::new(conf).await.unwrap();
    //         timeout(test_duration, hermes.run())
    //             .await
    //             .unwrap_or_else(|_x| Ok(())) // this line gets rid of
    //             // the timeout result
    //             .unwrap();
    //     });

    //     tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

    //     let soc = "http://[::1]:50051";

    //     let mut auth_client = AuthServiceClient::connect(soc).await.unwrap();

    //     let recv_res = auth_client
    //         .register(RegisterReq {
    //             name: "test".to_string(),
    //             suffix: "1".to_string(),
    //             email: "test@hermes.com".to_string(),
    //             password: "test".to_string(),
    //         })
    //         .await
    //         .unwrap()
    //         .into_inner();

    //     let (recv_user, recv_token) = (
    //         recv_res.created_user.unwrap(),
    //         recv_res.token.unwrap().inner,
    //     );

    //     let send_res = auth_client
    //         .register(RegisterReq {
    //             name: "test".to_string(),
    //             suffix: "2".to_string(),
    //             email: "test2@hermes.com".to_string(),
    //             password: "test".to_string(),
    //         })
    //         .await
    //         .unwrap()
    //         .into_inner();

    //     let (send_user, send_token) = (
    //         send_res.created_user.unwrap(),
    //         send_res.token.unwrap().inner,
    //     );

    //     let participants: Vec<grpc::User> = vec![send_user.clone(), recv_user.clone()];

    //     let mut convo_client = ConvoServiceClient::connect(soc).await.unwrap();
    //     let mut create_convo_req = Request::new(CreateConvoReq {
    //         participants: Some(UserList {
    //             inner: participants,
    //         }),
    //     });

    //     create_convo_req
    //         .metadata_mut()
    //         .insert("authorization", send_token.parse().unwrap());

    //     let convo = convo_client
    //         .create(create_convo_req)
    //         .await
    //         .unwrap()
    //         .into_inner()
    //         .created
    //         .unwrap();

    //     let send_user_cpy = send_user.clone();
    //     let reciever = tokio::spawn(async move {
    //         let mut client = MsgServiceClient::connect(soc).await.unwrap();
    //         let mut recv_req = RecieveRequest {
    //             author: "doesnt matter".to_string(),
    //         }
    //         .into_request();

    //         recv_req
    //             .metadata_mut()
    //             .insert("authorization", recv_token.parse().unwrap());

    //         let mut msg_stream = client
    //             .recieve_incoming(recv_req)
    //             .await
    //             .unwrap()
    //             .into_inner();

    //         let mut count = 0;
    //         while let Some(y) = timeout(test_duration, msg_stream.next())
    //             .await
    //             .ok()
    //             .flatten()
    //         {
    //             let msg = y.expect("message is cooked");
    //             assert_eq!(msg.body, "holy penis");
    //             assert_eq!(msg.author.unwrap(), send_user_cpy);
    //             count += 1;
    //         }

    //         assert_eq!(count, message_count); // making sure we didnt lose any messages
    //     });

    //     tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    //     let sender = tokio::spawn(async move {
    //         let mut client = MsgServiceClient::connect(soc).await.unwrap();

    //         for _ in 0..message_count {
    //             let mut req = SendMsgReq {
    //                 convo: Some(convo.clone()), //NOTE: idk why i need to clone here ima be honest
    //                 author: Some(send_user.clone()), // here too
    //                 body: "holy penis".to_string(),
    //                 unix_time: 123456789,
    //             }
    //             .into_request();
    //             req.metadata_mut()
    //                 .insert("authorization", send_token.parse().unwrap());
    //             let _msg = client.send(req).await.unwrap();
    //         }
    //     });

    //     let (server, reciever, sender) = tokio::join!(server, reciever, sender);

    //     server.expect("Server Panic");
    //     reciever.expect("Reciever Panic");
    //     sender.expect("Sender Panic");
    //     Ok(())
}
