use std::{collections::VecDeque, sync::Arc};

use agora::{TROJAN_IP, TROJAN_PORT};
use athens::grpc::{
    Ack, Packet, PacketList, ProxyFetchReq, proxy_service_server::ProxyService,
    trojan_service_client::TrojanServiceClient,
};
use color_eyre::eyre::Result;
use log::error;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status, async_trait, transport::Channel};

use crate::Devices;

pub struct ProxyServer {
    devices: Devices,
    trojan_client: Arc<Mutex<TrojanServiceClient<Channel>>>, // sparta_client: Arc<Mutex<MessageServiceClient<Channel>>>,
}

impl ProxyServer {
    pub async fn new(devices: Devices) -> Result<Self> {
        let server_url = format!("http://{}:{}", TROJAN_IP, TROJAN_PORT);

        let trojan_client = TrojanServiceClient::connect(server_url).await?;
        Ok(ProxyServer {
            devices,
            trojan_client: Arc::new(Mutex::new(trojan_client)),
        })
    }
}

#[async_trait]
impl ProxyService for ProxyServer {
    async fn send(self: Arc<Self>, req: Request<Packet>) -> Result<Response<Ack>, Status> {
        let mut client = self.trojan_client.lock().await;
        let ack = client.send(req.into_inner()).await?.into_inner();

        Ok(Response::new(ack))
    }

    async fn fetch(
        self: Arc<Self>,
        req: Request<ProxyFetchReq>,
    ) -> Result<Response<PacketList>, Status> {
        let (headers, _, req) = req.into_parts();

        let dev_id = headers
            .get("dev_id")
            .ok_or_else(|| error!("Device Id was not passed into headers!"))
            .map_err(|e| Status::internal("dw about it"))?
            .to_str()
            .map_err(|e| {
                error!("{}", e);
                Status::internal("dw about it")
            })?
            .parse::<usize>()
            .map_err(|e| {
                error!("{}", e);
                Status::internal("dw about it")
            })?;

        let mut devices = self.devices.lock().await;

        let mut ret = Vec::new();

        let device = devices.get_mut(dev_id).expect("device should exist");

        while let Some(msg) = device.message_queue.pop_front() {
            ret.push(msg);
        }

        while device.dummy_messages > 0 {
            ret.push(Packet {
                recipient: "dummy".to_string(),
                body: Vec::new(),
            });

            device.dummy_messages -= 1;
        }

        //TODO: limit how many "large" requests can be sent.
        Ok(Response::new(PacketList { inner: ret }))
    }
}
