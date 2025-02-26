// message that you want to send
//
// person you want to send it to
//
// address book synchronization would be pretty cool
// - happened in groovy
//

mod client;
mod grpc {
    tonic::include_proto!("hermes");
}
use clap::Parser;
use color_eyre::eyre::{eyre, Result};
use grpc::Message;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The message you would like to send
    #[arg(short, long)]
    message: String,

    /// The recipient id
    #[arg(short, long)]
    recipient_id: u32,
}

static SERVER_URL: &str = "http://[::1]:50051";

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    let msg = args.message.into_bytes();

    if msg.len() > 100 {
        return Err(eyre!("Message too long!"));
    }

    let mut msg_client = grpc::message_service_client::MessageServiceClient::connect(SERVER_URL)
        .await
        .unwrap();

    // need to  pad message into 100 bytes array
    let mut msg_buf = [0; 232];

    msg_buf[0..msg.len()].copy_from_slice(msg.as_slice());

    //NOTE: ideally we would now encrypt this buffer to send to the server,
    //with some sort of key thing we exchanged with the person we want to
    //communicate with

    println!("{:#?}", msg_buf);

    msg_client
        .send(Message {
            recipient: args.recipient_id.to_string(),
            body: msg_buf.to_vec(),
        })
        .await?;

    Ok(())
    // we need a grpc client
}
