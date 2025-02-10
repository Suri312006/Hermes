use color_eyre::eyre::Result;
use sparta::Sparta;

mod log;
mod sparta;
mod grpc {
    tonic::include_proto!("hermes");
}
mod services;
mod structures;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let server = Sparta::new()?;

    server.run().await
}
