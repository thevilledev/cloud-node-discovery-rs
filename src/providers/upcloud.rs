#![cfg(feature = "upcloud")]
use crate::Provider;
use crate::{DiscoveryError, Node};
use upcloud_sdk::client::Client;
use upcloud_sdk::resources::server::ServerOperations;
use upcloud_sdk::types::common::LabelFilter;

use upcloud_sdk::error::Error as UpcloudError;

use std::collections::HashMap;

use async_trait::async_trait;

pub struct UpcloudProvider {
    client: Client,
    zone: String,
    label_key: String,
    label_value: String,
}

impl UpcloudProvider {
    pub async fn new(config: &HashMap<String, String>) -> Result<Self, DiscoveryError> {
        // Extract required configuration
        let zone = config
            .get("zone")
            .ok_or_else(|| DiscoveryError::ConfigError("zone is required".to_string()))?
            .clone();
        let label_key = config
            .get("label_key")
            .ok_or_else(|| DiscoveryError::ConfigError("label_key is required".to_string()))?
            .clone();
        let label_value = config
            .get("label_value")
            .ok_or_else(|| DiscoveryError::ConfigError("label_value is required".to_string()))?
            .clone();

        // Initialize AWS SDK client with region
        let client = Client::new()?;

        Ok(UpcloudProvider {
            client,
            zone,
            label_key,
            label_value,
        })
    }
}

#[async_trait]
impl Provider for UpcloudProvider {
    async fn discover(&self) -> Result<Vec<Node>, DiscoveryError> {
        let label_filter = LabelFilter::new().with(&self.label_key, &self.label_value);
        let servers_by_labels = self.client.list_servers_by_labels(&label_filter).await?;

        // Transform instances into nodes
        let mut nodes = Vec::new();

        for server in servers_by_labels.server {
            let server_details = self.client.get_server(&server.uuid).await?;
            // Skip instances without a private IP
            let mut private_ip: &str = "";
            let mut utility_ip: &str = "";
            let mut public_ip: &str = "";
            for interface_iter in server_details.networking.iter() {
                for interface in interface_iter.interfaces.interface.iter() {
                    if interface.interface_type == "private" {
                        private_ip = interface.ip_addresses.ip_address[0].address.as_ref().unwrap().as_str();
                    }
                    if interface.interface_type == "utility" {
                        utility_ip = interface.ip_addresses.ip_address[0].address.as_ref().unwrap().as_str();
                    }
                    if interface.interface_type == "public" {
                        public_ip = interface.ip_addresses.ip_address[0].address.as_ref().unwrap().as_str();
                    }
                }
            }

            // Collect instance tags into metadata
            let mut metadata = HashMap::new();
            for x in server.labels.iter() {
                for label in x.label.iter() {
                    metadata.insert(label.key.to_string(), label.value.to_string());
                }
            }

            // Add instance ID and region to metadata
            metadata.insert("uuid".to_string(), server.uuid.to_string());
            metadata.insert("zone".to_string(), server.zone.clone());
            metadata.insert("utility_ip".to_string(), utility_ip.to_string());
            metadata.insert("public_ip".to_string(), public_ip.to_string());
            metadata.insert("zone".to_string(), self.zone.clone());

            nodes.push(Node {
                address: private_ip.to_string(),
                meta: metadata,
            });
        }

        Ok(nodes)
    }
}

impl From<UpcloudError> for DiscoveryError {
    fn from(error: UpcloudError) -> Self {
        DiscoveryError::ProviderError(error.to_string())
    }
}