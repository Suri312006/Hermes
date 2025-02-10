use color_eyre::eyre::Result;
use sparta::Sparta;

mod log;
mod sparta;
mod grpc {
    tonic::include_proto!("hermes");
}
mod services;
mod structures;

mod primitives;

#[tokio::main]
async fn main() -> Result<()> {
    // TODO: need to implement tls for secure client communication
    // https://github.com/hyperium/tonic/blob/master/examples/src/tls/client.rs
    color_eyre::install()?;

    let server = Sparta::new()?;

    server.run().await
}
