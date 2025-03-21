use agora::Log;
use agora::TROJAN_IP;
use agora::TROJAN_PORT;
use athens::grpc::NewUserReq;
use athens::grpc::Packet;
use athens::grpc::trojan_service_client::TrojanServiceClient;
use color_eyre::owo_colors::OwoColorize;
use ed25519_dalek::VerifyingKey;
use ed25519_dalek::ed25519::signature::SignerMut;
use ed25519_dalek::pkcs8::DecodePublicKey;
use state::State;

use std::{collections::VecDeque, sync::Arc, time::Duration};
use tonic::IntoRequest;

use athens::grpc::FetchReq;
use bincode::serde::encode_to_vec;
use clap::{Parser, Subcommand};
use color_eyre::eyre::{Result, eyre};
use log::info;
use rand_core::OsRng;
use server::Proxy;
use tokio::{select, spawn, sync::Mutex, task::JoinHandle, time::sleep};

mod auth;
mod server;
mod service;
mod state;

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

    /// stores device public keys to verify fetch requests
    AddDevice {
        #[arg(short, long)]
        key: String,
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

struct Device {
    pub_key: VerifyingKey,
    message_queue: VecDeque<Packet>,
    dummy_messages: u32,
}

impl Device {
    fn new(pub_key: VerifyingKey) -> Self {
        Device {
            pub_key,
            message_queue: VecDeque::new(),
            dummy_messages: 0,
        }
    }
}

type Devices = Arc<Mutex<Vec<Device>>>;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    Log::init()?;
    let args = Args::parse();

    let client_url = format!("http://{}:{}", TROJAN_IP, TROJAN_PORT);

    let mut client = TrojanServiceClient::connect(client_url).await?;

    match args.command {
        Commands::CreateUser {} => {
            let mut rng = OsRng;

            let signing_key = ed25519_dalek::SigningKey::generate(&mut rng);
            let verifying_key = signing_key.verifying_key();

            let encoded_key = encode_to_vec(verifying_key, bincode::config::standard())?;

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

            let _ = State::new(user_id, signing_key)?;
            println!("{}", "User Creation Successfull!".green());
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
            //TODO: do dis shi
            let mut state = State::read()?;

            let mut device_vec = Vec::new();

            for dev_key in state.device_pub_keys {
                device_vec.push(Device::new(dev_key))
            }

            let devices: Devices = Arc::new(Mutex::new(device_vec));

            let closure_devices = devices.clone();

            let recipient_sig = state.user_key.sign(state.user_id.as_bytes());

            // spawn a task that keeps pulling from sparta on a regular interval
            let handle: JoinHandle<Result<()>> = spawn(async move {
                loop {
                    if let Some(msg) = client
                        .fetch(FetchReq {
                            recipient: state.user_id.clone(),
                            amount: 1,
                            sig: recipient_sig.to_vec(),
                        })
                        .await?
                        .into_inner()
                        .inner
                        .first()
                    {
                        info!("pulling!");
                        //NOTE: this only works if adversary cannot observe the plaintext of the communication link between the enclave and the recipient
                        if msg.recipient == state.user_id {
                            let mut device_vec = closure_devices.lock().await;
                            for device in device_vec.iter_mut() {
                                device.message_queue.push_back(msg.clone());
                            }
                        }
                    } else {
                        let mut device_vec = closure_devices.lock().await;
                        for device in device_vec.iter_mut() {
                            device.dummy_messages += 1;
                        }
                    }

                    sleep(delay_time).await;
                }
            });

            let server = Proxy::new(devices).await?;

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
        Commands::AddDevice { key } => {
            // key needs to be parsed as a verifying key
            let formatted_dev_key = format!(
                "-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----",
                key
            );

            let verifying_key = VerifyingKey::from_public_key_pem(&formatted_dev_key)?;

            let state = State::read()?;
            let _ = state.add_device_pub_key(verifying_key)?;

            println!("{}", "Successfully added device!".green());

            return Ok(());
        }
    }
}
