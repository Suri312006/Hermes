// message that you want to send
//
// person you want to send it to
//
// address book synchronization would be pretty cool
// - happened in groovy
//

use std::time::Duration;

use athens::{
    client::SpartaClient,
    grpc::{FetchReq, NewUserReq, Packet, user_service_client::UserServiceClient},
};
use clap::Parser;
use color_eyre::eyre::{Result, eyre};
use tokio::time::sleep;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    granularity: TimeGranularity,

    #[arg(short, long)]
    messages_per_time_step: u64,
}

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
enum TimeGranularity {
    Week,
    Day,
    Hour,
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

    loop {
        client
            .msg_client
            .fetch(FetchReq {
                recipient: "0000".to_string(),
                amount: 1,
            })
            .await?;
        sleep(delay_time).await;
    }
}
