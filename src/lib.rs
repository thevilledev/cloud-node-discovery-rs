use async_trait::async_trait;
use serde::Deserialize;
use std::collections::HashMap;
use thiserror::Error;

use crate::config::parse_config;

mod config;
mod providers;

#[derive(Debug, Error)]
pub enum DiscoveryError {
    #[error("Invalid configuration: {0}")]
    ConfigError(String),
    #[error("Provider error: {0}")]
    ProviderError(String),
    #[error("Unknown provider: {0}")]
    UnknownProvider(String),
}

#[derive(Debug, Deserialize)]
pub struct Node {
    pub address: String,
    pub meta: HashMap<String, String>,
}

#[async_trait]
pub trait Provider {
    async fn discover(&self) -> Result<Vec<Node>, DiscoveryError>;
}

pub struct Discovery {
    provider: Box<dyn Provider>,
}

impl Discovery {
    pub async fn new(provider_name: &str, config: &str) -> Result<Self, DiscoveryError> {
        let config = parse_config(config)?;
        println!("{:?}", config);
        let provider: Box<dyn Provider> = match provider_name {
            #[cfg(feature = "aws")]
            "aws" => Box::new(providers::aws::AwsProvider::new(&config).await?),
            _ => return Err(DiscoveryError::UnknownProvider(provider_name.to_string())),
        };

        Ok(Discovery { provider })
    }

    pub async fn discover(&self) -> Result<Vec<Node>, DiscoveryError> {
        self.provider.discover().await
    }
}
