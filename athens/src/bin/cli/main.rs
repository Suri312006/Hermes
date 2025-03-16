use std::process::Command;

use agora::SPARTA_PORT;
use args::{CliArgs, Commands};
use athens::{
    config::Config,
    grpc::{
        FetchReq, NewUserReq, Packet, message_service_client::MessageServiceClient,
        user_service_client::UserServiceClient, user_service_server,
    },
};
use clap::Parser;
use color_eyre::{
    eyre::{Result, eyre},
    owo_colors::OwoColorize,
};
use tonic::IntoRequest;

mod args;
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let server_url = format!("http://{}", SPARTA_PORT);

    let args = CliArgs::parse();

    match args.command {
        Commands::Init {} => {
            let mut user_client = UserServiceClient::connect(server_url).await?;
            let user_id = user_client
                .create_user(NewUserReq::default().into_request())
                .await?
                .into_inner()
                .id;

            let _ = Config::new(user_id)?;

            println!("{}", "Initialization Successfull!".green())
        }
        Commands::Message(args) => {

            
            
            // send message stuff
        }
        Commands::Contacts(args) => {
            // add contacts
        }
    }

    // let msg = args.message.into_bytes();

    // if msg.len() > 100 {
    //     return Err(eyre!("Message too long!"));
    // }

    // let mut user_client = UserServiceClient::connect(server_url.clone()).await?;

    // let mut msg_client = MessageServiceClient::connect(server_url).await.unwrap();

    // // need to  pad message into 100 bytes array
    // let mut msg_buf = [0; 232];

    // msg_buf[0..msg.len()].copy_from_slice(msg.as_slice());

    // //NOTE: ideally we would now encrypt this buffer to send to the server,
    // //with some sort of key thing we exchanged with the person we want to
    // //communicate with

    // println!("{:#?}", msg_buf);

    // let i = user_client
    //     .create_user(NewUserReq {}.into_request())
    //     .await?
    //     .into_inner()
    //     .id;

    // println!("got id!: {}", id);

    // msg_client
    //     .send(Packet {
    //         recipient: id.clone(),
    //         body: msg_buf.to_vec(),
    //     })
    //     .await?;

    // let msgs = msg_client
    //     .fetch(FetchReq {
    //         recipient: id,
    //         amount: 2,
    //     })
    //     .await?
    //     .into_inner();

    // for msg in msgs.inner {
    //     println!(
    //         "recipient: {:#?}, message: {:#?}",
    //         msg.recipient,
    //         String::from_utf8(msg.body)
    //     );
    // }

    // Ok(())
    Ok(())
    // we need a grpc client
}
