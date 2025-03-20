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

use crate::{auth::AuthInterceptor, service::ProxyServer};
use tonic_middleware::InterceptorFor;

pub struct Proxy {
    router: Router,
}

impl Proxy {
    pub async fn new(messages_vec: Arc<Mutex<Vec<VecDeque<Packet>>>>) -> Result<Self> {
        let router = Server::builder().add_service(InterceptorFor::new(
            ProxyServiceServer::new(ProxyServer::new(messages_vec.clone()).await?),
            AuthInterceptor::new(),
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
