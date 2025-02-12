use color_eyre::eyre::Result;
use sparta::{Log, Sparta};

#[tokio::main]
async fn main() -> Result<()> {
    // TODO: need to implement tls for secure client communication
    // https://github.com/hyperium/tonic/blob/master/examples/src/tls/client.rs
    color_eyre::install()?;
    Log::init()?;
    let server = Sparta::new()?;

    server.run().await
}
