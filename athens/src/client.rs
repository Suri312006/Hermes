use color_eyre::eyre::Result;
use tonic::transport::Channel;

use crate::grpc::{
    message_service_client::MessageServiceClient, user_service_client::UserServiceClient,
};

pub struct SpartaClient {
    pub user_client: UserServiceClient<Channel>,
    pub msg_client: MessageServiceClient<Channel>,
}

impl SpartaClient {
    pub async fn new(server_url: String) -> Result<Self> {
        Ok(SpartaClient {
            user_client: UserServiceClient::connect(server_url.clone()).await?,
            msg_client: MessageServiceClient::connect(server_url).await?,
        })
    }

    pub async fn default() -> Result<Self> {
        let server_url = format!("http://{}", agora::SPARTA_PORT);

        Ok(SpartaClient {
            user_client: UserServiceClient::connect(server_url.clone()).await?,
            msg_client: MessageServiceClient::connect(server_url).await?,
        })
    }
}
