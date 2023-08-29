pub mod config;

use std::sync::Arc;

use async_trait::async_trait;
pub use config::Config;
use domain::modules::shared::errors::UnexpectedError;
use domain::map_unexpected_error;
use integrator::MsgBrokerChannel;
use lapin::{
    options::{BasicPublishOptions, ExchangeDeclareOptions},
    types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties, ExchangeKind,
};

pub async fn new_connection(config: Config) -> Result<Connection, lapin::Error> {
    Connection::connect(&config.url, ConnectionProperties::default()).await
}

pub struct MsgBrokerImpl {
    connection: Arc<Connection>,
}

impl MsgBrokerImpl {
    pub fn new(connection: Arc<Connection>) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl integrator::MsgBroker for MsgBrokerImpl {
    async fn create_channel(&self) -> Result<Box<dyn MsgBrokerChannel>, UnexpectedError> {
        let rabbit_channel = self
            .connection
            .as_ref()
            .create_channel()
            .await
            .map_err(|err| map_unexpected_error!(err))?;
        Ok(Box::new(MsgBrokerChannelImpl {
            channel: rabbit_channel,
        }))
    }
}

pub struct MsgBrokerChannelImpl {
    channel: Channel,
}

#[async_trait]
impl integrator::MsgBrokerChannel for MsgBrokerChannelImpl {
    async fn send(&self, key: &str, serialized_msg: &str) -> Result<(), UnexpectedError> {
        self.channel
            .exchange_declare(
                key,
                ExchangeKind::Fanout,
                ExchangeDeclareOptions {
                    durable: true,
                    ..ExchangeDeclareOptions::default()
                },
                FieldTable::default(),
            )
            .await
            .map_err(|err| map_unexpected_error!(err))?;

        self.channel
            .basic_publish(
                key,
                "",
                BasicPublishOptions::default(),
                serialized_msg.as_bytes(),
                BasicProperties::default().with_delivery_mode(2),
            )
            .await
            .map_err(|err| map_unexpected_error!(err))?;

        Ok(())
    }
}
