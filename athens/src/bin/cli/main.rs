use agora::{MSG_SIZE, PROXY_PORT};
use args::{CliArgs, Commands, MessageSubCommands};
use athens::grpc::{Packet, ProxyFetchReq, proxy_service_client::ProxyServiceClient};
use clap::Parser;
use color_eyre::{
    eyre::{Result, eyre},
    owo_colors::OwoColorize,
};
use rand_core::OsRng;

use ed25519_dalek::pkcs8::EncodePublicKey;

mod args;
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let proxy_url = format!("http://{}", PROXY_PORT);

    let args = CliArgs::parse();

    match args.command {
        Commands::Register {} => {
            let mut rng = OsRng;

            // now we have to store this key
            let signing_key = ed25519_dalek::SigningKey::generate(&mut rng);

            let verifying_key = signing_key.verifying_key();

            let verifying_key = verifying_key
                .to_public_key_pem(ed25519_dalek::pkcs8::spki::der::pem::LineEnding::LF)?;

            let mut iter = verifying_key.split("\n");
            iter.next();

            println!(
                "Registration Key: {}",
                iter.next().ok_or(eyre!("Weird verifying key"))?.green()
            );

            println!("Next: Copy the above key into your proxy as follows");
            println!("\nproxy add-device -k {}", "<KEY>".green());
        }

        Commands::Message(subcommand) => {
            let mut proxy_client = ProxyServiceClient::connect(proxy_url).await?;
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
                MessageSubCommands::Fetch => {
                    let msgs = proxy_client.fetch(ProxyFetchReq {}).await?.into_inner();
                    for msg in msgs.inner {
                        println!(
                            "From: {:?}\nBody: {:?}",
                            msg.recipient,
                            String::from_utf8(msg.body)
                        );
                    }
                }
            }
            // send message stuff
        }
        Commands::Contacts(args) => {
            // add contacts
            todo!("Implement contacts");
        }
    }

    Ok(())
    // we need a grpc client
}
