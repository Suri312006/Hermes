// message that you want to send
//
// person you want to send it to
//
// address book synchronization would be pretty cool
// - happened in groovy
//

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use athens::{client::SpartaClient, config::Config, grpc::FetchReq};
use clap::Parser;
use color_eyre::eyre::{Context, Result, eyre};
use tokio::{spawn, task::JoinHandle, time::sleep};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    granularity: TimeGranularity,

    #[arg(short, long, default_value_t = 30)]
    messages_per_time_step: u64,
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
    let args = Args::parse();

    let delay_time = match args.granularity {
        TimeGranularity::Week => {
            let seconds_per_week = 604800;
            Duration::from_millis(seconds_per_week * 1000 / args.messages_per_time_step)
        }
        TimeGranularity::Day => {
            let seconds_per_day = 86400;
            Duration::from_millis(seconds_per_day * 1000 / args.messages_per_time_step)
        }
        TimeGranularity::Hour => {
            let seconds_per_hour = 3600;
            Duration::from_millis(seconds_per_hour * 1000 / args.messages_per_time_step)
        }
        TimeGranularity::Minute => {
            let seconds_per_minute = 60;
            Duration::from_millis(seconds_per_minute * 1000 / args.messages_per_time_step)
        }

        TimeGranularity::Second => Duration::from_millis(1000 / args.messages_per_time_step),
    };

    if delay_time < Duration::from_secs(2) {
        return Err(eyre!(
            "You are trying to fetch too often, please reduce the amount of messages you would like to fetch."
        ));
    }

    // now we set up a worker thread that fetches for delay
    let mut client = SpartaClient::default().await?;
    let config = Config::read()?;

    let real_msgs = Arc::new(Mutex::new(Vec::new()));

    // spawn a task that keeps pulling from sparta on a regular interval
    let handle: JoinHandle<Result<()>> = spawn(async move {
        loop {
            if let Some(msg) = client
                .msg_client
                .fetch(FetchReq {
                    recipient: config.user_id.clone(),
                    amount: 1,
                })
                .await?
                .into_inner()
                .inner
                .first()
            {
                //NOTE: this only works if adversary cannot observe the plaintext of the communication link between the enclave and the recipient
                if msg.recipient == config.user_id {
                    real_msgs
                        .lock()
                        .map_err(|e| eyre!("{:?}", e))
                        .with_context(|| "Please re-run the proxy!")?
                        .push(msg.clone());
                }
            }

            sleep(delay_time).await;
        }
    });

    handle.await??;

    Ok(())
}
