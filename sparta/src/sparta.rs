use std::{
    net::SocketAddr,
    str::FromStr,
    sync::{Arc, Mutex},
};

use color_eyre::eyre::{eyre, Result};
use tonic::{
    server::ServerStreamingService,
    transport::{server::Router, Server},
};

use crate::{
    grpc::message_service_server::MessageServiceServer,
    messagestore::{MessageStore, MessageStoreInner},
    services::MessageServer,
    user_service_server::UserServiceServer,
    userstore::UserStoreInner,
    Log, UserServer,
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
        let socket = SocketAddr::from_str("[::1]:50051").expect("Parsing Socket Address Failed!");
        println!("Server Listening at {}!", socket);
        self.router.serve(socket).await.map_err(|e| eyre!(e))
    }
}
