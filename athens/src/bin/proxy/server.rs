use std::{collections::VecDeque, net::SocketAddr, str::FromStr, sync::Arc};

use agora::Log;
use athens::grpc::{Packet, proxy_service_server::ProxyServiceServer};
use color_eyre::eyre::{Result, eyre};
use log::info;
use tokio::sync::Mutex;
use tonic::transport::{
    Server,
    server::{Router, TcpConnectInfo},
};

use crate::service::ProxyServer;

pub struct Proxy {
    router: Router,
}

impl Proxy {
    pub async fn new(messages_vec: Arc<Mutex<VecDeque<Packet>>>) -> Result<Self> {
        // Log::init()?;

        let router = Server::builder().add_service(ProxyServiceServer::new(
            ProxyServer::new(messages_vec.clone()).await?,
        ));

        Ok(Proxy { router })
    }

    pub async fn run(self) -> Result<()> {
        let socket =
            SocketAddr::from_str(agora::PROXY_PORT).expect("Parsing Socket Address Failed!");
        info!("Proxy Listening at {}!", socket);
        self.router.serve(socket).await.map_err(|e| eyre!(e))
    }
}
