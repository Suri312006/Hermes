use agora::{MSG_SIZE, PROXY_IP, PROXY_PORT, VERIFY_PHRASE};
use args::{CliArgs, Commands, MessageSubCommands};
use athens::grpc::{Packet, ProxyFetchReq, proxy_service_client::ProxyServiceClient};
use clap::Parser;
use color_eyre::{
    eyre::{Result, eyre},
    owo_colors::OwoColorize,
};
use rand_core::OsRng;

use ed25519_dalek::{ed25519::signature::SignerMut, pkcs8::EncodePublicKey};
use state::State;
use tonic::{IntoRequest, metadata::MetadataValue};

mod args;
mod state;
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let proxy_url = format!("http://{}:{}", PROXY_IP, PROXY_PORT);

    let args = CliArgs::parse();

    match args.command {
        Commands::Register {} => {
            let mut rng = OsRng;

            // now we have to store this key
            let signing_key = ed25519_dalek::SigningKey::generate(&mut rng);

            // client has state too?

            let verifying_key = signing_key.verifying_key();

            let verifying_key = verifying_key
                .to_public_key_pem(ed25519_dalek::pkcs8::spki::der::pem::LineEnding::LF)?;

            let mut iter = verifying_key.split("\n");
            iter.next();

            State::new(signing_key);

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

                    let mut msg = vec![];

                    msg.resize(MSG_SIZE, 1);

                    for byte in message.as_bytes() {
                        msg.push(*byte)
                    }

                    let mut req = Packet {
                        recipient,
                        body: msg,
                    }
                    .into_request();

                    println!("Sending this message: {:?}", req);

                    let mut state = State::read()?;

                    let sig = state.signing_key.sign(VERIFY_PHRASE.as_bytes());

                    let sign_str = hex::encode(sig.to_bytes());

                    req.metadata_mut().insert("signature", sign_str.parse()?);
                    //TODO: okay we need to chunk this guy up.
                    proxy_client.send(req).await?;
                }
                MessageSubCommands::Fetch => {
                    let mut req = ProxyFetchReq {}.into_request();
                    let mut state = State::read()?;

                    let sig = state.signing_key.sign(VERIFY_PHRASE.as_bytes());

                    let sign_str = hex::encode(sig.to_bytes());

                    req.metadata_mut().insert("signature", sign_str.parse()?);

                    let msgs = proxy_client.fetch(req).await?.into_inner();
                    for msg in msgs.inner {
                        println!(
                            "To: {:?}\nBody: {:?}",
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
            // okayyyyy so basically we can just worry about this later
        }
    }

    Ok(())
    // we need a grpc client
}
