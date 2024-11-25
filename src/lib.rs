//! Cloud Node Discovery for Rust
//!
//! This crate provides functionality to discover nodes in various cloud environments.
//! Currently supports:
//!
//! - AWS EC2 instances (with tag-based filtering)
//!
//! # Example
//!
//! ```rust
//! use cloud_node_discovery::{Discovery, DiscoveryError};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), DiscoveryError> {
//!     let discovery = Discovery::new("aws", "region=us-east-1,tag_key=foo,tag_value=bar").await?;
//!     let nodes = discovery.discover().await?;
//!     println!("{:?}", nodes);
//!     Ok(())
//! }
//! ```
//!
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

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
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
            #[cfg(feature = "upcloud")]
            "upcloud" => Box::new(providers::upcloud::UpcloudProvider::new(&config).await?),
            _ => return Err(DiscoveryError::UnknownProvider(provider_name.to_string())),
        };

        Ok(Discovery { provider })
    }

    pub async fn discover(&self) -> Result<Vec<Node>, DiscoveryError> {
        self.provider.discover().await
    }
}

pub struct DiscoveryBuilder {
    provider_name: String,
    config: HashMap<String, String>,
}

impl DiscoveryBuilder {
    pub fn new(provider_name: &str) -> Self {
        Self {
            provider_name: provider_name.to_string(),
            config: HashMap::new(),
        }
    }

    pub fn with_config(mut self, key: &str, value: &str) -> Self {
        self.config.insert(key.to_string(), value.to_string());
        self
    }

    pub async fn build(self) -> Result<Discovery, DiscoveryError> {
        Discovery::new(&self.provider_name, &self.config_to_string()).await
    }

    fn config_to_string(&self) -> String {
        self.config
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join(",")
    }
}
