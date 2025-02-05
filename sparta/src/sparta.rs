use std::{net::SocketAddr, str::FromStr};

use color_eyre::eyre::{eyre, Result};
use tonic::{
    server::ServerStreamingService,
    transport::{server::Router, Server},
};

use crate::{grpc::message_service_server::MessageServiceServer, services::MessageServer};

pub struct Sparta {
    router: Router,
}

impl Sparta {
    pub fn new() -> Self {
        let router = Server::builder().add_service(MessageServiceServer::new(MessageServer::new()));
        Sparta { router }
    }

    pub async fn run(self) -> Result<()> {
        //TODO: probably want to make the port a config option
        let socket = SocketAddr::from_str("[::1]:50051").expect("Parsing Socket Address Failed!");
        println!("Server Listening at {}!", socket);
        self.router.serve(socket).await.map_err(|e| eyre!(e))
    }
}
