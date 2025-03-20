use std::net::SocketAddr;

use std::str::FromStr;
use std::sync::Arc;

use agora::{Log, SPARTA_VSOCK_CID, SPARTA_VSOCK_PORT, TROJAN_BIND_ADDR, TROJAN_PORT};
use color_eyre::eyre::{Result, eyre};
use grpc::trojan_service_server::{TrojanService, TrojanServiceServer};
use grpc::{Ack, FetchReq, NewUserReq, NewUserRes, Packet, PacketList};
use grpc::{message_service_client::MessageServiceClient, user_service_client::UserServiceClient};
use log::{info, trace};
use tonic::transport::{Channel, Server};

use hyper_util::rt::TokioIo;
use tokio_vsock::{VsockAddr, VsockStream};
use tonic::transport::{Endpoint, Uri};
use tonic::{IntoRequest, Request, Response, Status, async_trait};

use tokio::sync::Mutex;
mod grpc {
    tonic::include_proto!("hermes");
}

#[tokio::main]
async fn main() -> Result<()> {
    Log::init()?;
    color_eyre::install()?;
    let trojan_server = TrojanServer::new().await?;
    let router = Server::builder().add_service(TrojanServiceServer::new(trojan_server));

    let sock = match SocketAddr::from_str(format!("{}:{}", TROJAN_BIND_ADDR, TROJAN_PORT).as_str())
    {
        Ok(sock) => sock,
        Err(e) => {
            println!(
                "{}",
                format!("{}:{}", TROJAN_BIND_ADDR, TROJAN_PORT).as_str()
            );
            return Err(eyre!(e));
        }
    };

    info!("Listening on {}!", sock);
    router.serve(sock).await.map_err(|err| eyre!(err))
}

pub struct TrojanServer {
    user_client: Arc<Mutex<UserServiceClient<Channel>>>,
    msg_client: Arc<Mutex<MessageServiceClient<Channel>>>,
}

impl TrojanServer {
    pub async fn new() -> Result<Self> {
        let endpoint = Endpoint::from_static("http://localhost")
            .connect_with_connector(tower::service_fn(|_: Uri| async {
                Ok::<_, std::io::Error>(TokioIo::new(
                    VsockStream::connect(VsockAddr::new(SPARTA_VSOCK_CID, SPARTA_VSOCK_PORT))
                        .await?,
                ))
            }))
            .await
            .unwrap();
        trace!("connected to sparta vsock endpoint");

        let user_client = UserServiceClient::new(endpoint.clone());
        let msg_client = MessageServiceClient::new(endpoint);

        trace!("Connected user and msg clients to vsock");

        Ok(TrojanServer {
            user_client: Arc::new(Mutex::new(user_client)),
            msg_client: Arc::new(Mutex::new(msg_client)),
        })
    }
}

#[async_trait]
impl TrojanService for TrojanServer {
    async fn send(self: Arc<Self>, req: Request<Packet>) -> Result<Response<Ack>, Status> {
        trace!("recieved send");
        let mut client = self.msg_client.lock().await;
        client.send(req).await
    }

    async fn fetch(
        self: Arc<Self>,
        req: Request<FetchReq>,
    ) -> Result<Response<PacketList>, Status> {
        trace!("recieved fetch");
        let mut client = self.msg_client.lock().await;
        client.fetch(req).await
    }
    async fn create_user(
        self: Arc<Self>,
        req: Request<NewUserReq>,
    ) -> Result<Response<NewUserRes>, Status> {
        trace!("recieved create_user");
        let mut client = self.user_client.lock().await;
        client.create_user(req).await
    }
}
