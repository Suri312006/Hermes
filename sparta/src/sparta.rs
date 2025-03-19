use std::{
    net::SocketAddr,
    str::FromStr,
    sync::{Arc, Mutex},
};

use agora::Log;
use color_eyre::{
    eyre::{Result, eyre},
    owo_colors::OwoColorize,
};
use log::info;
use tokio_vsock::{VsockAddr, VsockListener};
use tonic::transport::{Server, server::Router};

use crate::{
    UserServer, grpc::message_service_server::MessageServiceServer,
    messagestore::MessageStoreInner, services::MessageServer,
    user_service_server::UserServiceServer, userstore::UserStoreInner,
};

pub struct Sparta {
    router: Router,
}

impl Sparta {
    pub fn new() -> Result<Self> {
        Log::init()?;

        let message_store = Arc::new(Mutex::new(MessageStoreInner::new()?));
        let user_store = Arc::new(Mutex::new(UserStoreInner::new()?));

        let router = Server::builder()
            .add_service(MessageServiceServer::new(MessageServer::new(
                &user_store,
                &message_store,
            )))
            .add_service(UserServiceServer::new(UserServer::new(&user_store)));
        Ok(Sparta { router })
    }

    pub async fn run(self) -> Result<()> {
        //TODO: probably want to make the port a config option
        let socket = VsockAddr::new(16, 50051);
        // let socket =
        //     SocketAddr::from_str(agora::SPARTA_PORT).expect("Parsing Socket Address Failed!");
        info!("Server Listening at {}!", socket);
        // self.router.serve(socket).await.map_err(|e| eyre!(e))

        // cid and 32 bit port number
        let listener = VsockListener::bind(socket)?;

        self.router
            .serve_with_incoming(listener.incoming())
            .await
            .map_err(|e| eyre!(e))
    }
}
