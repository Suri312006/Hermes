use color_eyre::eyre::Result;
use sparta::Sparta;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    // TODO: need to implement tls for secure client communication
    // https://github.com/hyperium/tonic/blob/master/examples/src/tls/client.rs

    color_eyre::install()?;

    let server = Sparta::new()?;

    server.run().await
}
