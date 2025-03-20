use agora::Log;
use agora::TROJAN_IP;
use agora::TROJAN_PORT;
use athens::grpc::NewUserReq;
use athens::grpc::trojan_service_client::TrojanServiceClient;
use ed25519_dalek::pkcs8::EncodePrivateKey;
use ed25519_dalek::pkcs8::EncodePublicKey;

use ed25519_dalek::{VerifyingKey, pkcs8};
use log::trace;
use std::{collections::VecDeque, sync::Arc, time::Duration};
use tonic::IntoRequest;

use athens::{config::Config, grpc::FetchReq};
use bincode::serde::encode_to_vec;
use clap::{Parser, Subcommand};
use color_eyre::eyre::{Result, eyre};
use log::info;
use rand_core::OsRng;
use server::Proxy;
use tokio::{select, spawn, sync::Mutex, task::JoinHandle, time::sleep};

mod server;
mod service;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    CreateUser {},

    Run {
        #[arg(short, long)]
        granularity: TimeGranularity,

        #[arg(short, long, default_value_t = 30)]
        messages_per_time_step: u64,
    },
}

#[derive(Clone, Copy, Debug, clap::ValueEnum, Default)]
enum TimeGranularity {
    Week,
    Day,
    Hour,
    #[default]
    Minute,
    Second,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    Log::init()?;
    let args = Args::parse();

    // let mut client = SpartaClient::default().await?;
    let client_url = format!("http://{}:{}", TROJAN_IP, TROJAN_PORT);
    trace!("client_url: {}", client_url);

    let mut client = TrojanServiceClient::connect(client_url).await?;

    match args.command {
        Commands::CreateUser {} => {
            let mut rng = OsRng;

            let signing_key = ed25519_dalek::SigningKey::generate(&mut rng);
            let verifying_key = signing_key.verifying_key();

            let encoded_key = encode_to_vec(verifying_key, bincode::config::standard())?;

            trace!("about to hit up client");

            let user_id = client
                .create_user(
                    NewUserReq {
                        public_key: encoded_key,
                    }
                    .into_request(),
                )
                .await?
                .into_inner()
                .id;

            trace!("got a resp");

            let out = signing_key.to_pkcs8_pem(pkcs8::spki::der::pem::LineEnding::LF)?;
            let x = verifying_key.to_public_key_pem(pkcs8::spki::der::pem::LineEnding::LF)?;

            // ok so now we need to store this signing key as a pem

            println!("public key:\n{x}");

            println!("user_id: {}\nprivate key: \n{}", user_id, out.as_str());
            Ok(())
        }

        Commands::Run {
            granularity,
            messages_per_time_step,
        } => {
            let delay_time = match granularity {
                TimeGranularity::Week => {
                    let seconds_per_week = 604800;
                    Duration::from_millis(seconds_per_week * 1000 / messages_per_time_step)
                }
                TimeGranularity::Day => {
                    let seconds_per_day = 86400;
                    Duration::from_millis(seconds_per_day * 1000 / messages_per_time_step)
                }
                TimeGranularity::Hour => {
                    let seconds_per_hour = 3600;
                    Duration::from_millis(seconds_per_hour * 1000 / messages_per_time_step)
                }
                TimeGranularity::Minute => {
                    let seconds_per_minute = 60;
                    Duration::from_millis(seconds_per_minute * 1000 / messages_per_time_step)
                }

                TimeGranularity::Second => Duration::from_millis(1000 / messages_per_time_step),
            };

            if delay_time < Duration::from_secs(2) {
                return Err(eyre!(
                    "You are trying to fetch too often, please reduce the amount of messages you would like to fetch."
                ));
            }

            // now we set up a worker thread that fetches for delay
            let config = Config::read()?;

            let msgs_queue = Arc::new(Mutex::new(VecDeque::new()));
            let closure_queue = msgs_queue.clone();

            // spawn a task that keeps pulling from sparta on a regular interval
            let handle: JoinHandle<Result<()>> = spawn(async move {
                loop {
                    if let Some(msg) = client
                        .fetch(FetchReq {
                            recipient: config.user_id.clone(),
                            amount: 1,
                            sig: String::new(),
                        })
                        .await?
                        .into_inner()
                        .inner
                        .first()
                    {
                        info!("pulling!");
                        //NOTE: this only works if adversary cannot observe the plaintext of the communication link between the enclave and the recipient
                        if msg.recipient == config.user_id {
                            closure_queue.lock().await.push_back(msg.clone());
                        }
                    }

                    sleep(delay_time).await;
                }
            });

            let server = Proxy::new(msgs_queue).await?;

            let resp = select! {

                val = server.run() => {
                    val
                }

                val2 = handle => {
                    val2?
                }

            };
            resp
        }
    }
}
