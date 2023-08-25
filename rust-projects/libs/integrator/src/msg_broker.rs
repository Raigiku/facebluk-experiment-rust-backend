use async_trait::async_trait;
use domain::modules::shared::errors::UnexpectedError;

#[async_trait]
pub trait MsgBroker: Sync + Send {
    async fn create_channel(&self) -> Result<Box<dyn MsgBrokerChannel>, UnexpectedError>;
}

#[async_trait]
pub trait MsgBrokerChannel: Sync + Send {
    async fn send(&self, key: &str, serialized_msg: &str) -> Result<(), UnexpectedError>;
}
