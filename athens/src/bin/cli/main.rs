use std::process::Command;

use agora::{MSG_SIZE, PROXY_PORT, SPARTA_PORT};
use args::{CliArgs, Commands, MessageSubCommands};
use athens::{
    config::Config,
    grpc::{
        FetchReq, NewUserReq, Packet, message_service_client::MessageServiceClient,
        proxy_service_client::ProxyServiceClient, user_service_client::UserServiceClient,
        user_service_server,
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

    let proxy_url = format!("http://{}", PROXY_PORT);

    let args = CliArgs::parse();

    let mut proxy_client = ProxyServiceClient::connect(proxy_url).await?;

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

        Commands::Message(subcommand) => {
            match subcommand {
                MessageSubCommands::Send {
                    message,
                    recipient,
                    path: _path,
                } => {
                    if message.len() > MSG_SIZE {
                        return Err(eyre!(
                            "Message Too Large! Message must be under {:?} bytes",
                            MSG_SIZE
                        ));
                    }
                    //TODO: okay we need to chunk this guy up.

                    proxy_client
                        .send(Packet {
                            recipient,
                            body: message.as_bytes().to_vec(),
                        })
                        .await?;
                }
                MessageSubCommands::Fetch => {}
            }
            // send message stuff
        }
        Commands::Contacts(args) => {
            // add contacts
        }
    }

    Ok(())
    // we need a grpc client
}
