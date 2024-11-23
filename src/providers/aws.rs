#[cfg(feature = "aws")]
use crate::Provider;
use crate::{DiscoveryError, Node};
use aws_sdk_ec2::types::Filter;
use aws_sdk_ec2::Client;
use std::collections::HashMap;

use async_trait::async_trait;

pub struct AwsProvider {
    client: Client,
    region: String,
    tag_key: String,
    tag_value: String,
}

impl AwsProvider {
    pub async fn new(config: &HashMap<String, String>) -> Result<Self, DiscoveryError> {
        // Extract required configuration
        let region = config
            .get("region")
            .ok_or_else(|| DiscoveryError::ConfigError("region is required".to_string()))?
            .clone();
        let tag_key = config
            .get("tag_key")
            .ok_or_else(|| DiscoveryError::ConfigError("tag_key is required".to_string()))?
            .clone();
        let tag_value = config
            .get("tag_value")
            .ok_or_else(|| DiscoveryError::ConfigError("tag_value is required".to_string()))?
            .clone();

        // Initialize AWS SDK client with region
        let config = aws_config::from_env()
            .region(aws_sdk_ec2::config::Region::new(region.clone()))
            .load()
            .await;
        let client = Client::new(&config);

        Ok(AwsProvider {
            client,
            region,
            tag_key,
            tag_value,
        })
    }
}

#[async_trait]
impl Provider for AwsProvider {
    async fn discover(&self) -> Result<Vec<Node>, DiscoveryError> {
        // Create tag filter
        let tag_filter = Filter::builder()
            .name(format!("tag:{}", self.tag_key))
            .values(self.tag_value.clone())
            .build();

        // Query EC2 instances with the filter
        let instances = self
            .client
            .describe_instances()
            .filters(tag_filter)
            .send()
            .await
            .map_err(|e| DiscoveryError::ProviderError(e.to_string()))?;

        // Transform instances into nodes
        let mut nodes = Vec::new();

        for reservation in instances.reservations() {
            for instance in reservation.instances() {
                // Skip instances without a private IP
                let private_ip = match instance.private_ip_address() {
                    Some(ip) => ip.to_string(),
                    None => continue,
                };

                // Collect instance tags into metadata
                let mut metadata = HashMap::new();
                for tag in instance.tags() {
                    if let (Some(key), Some(value)) = (tag.key(), tag.value()) {
                        metadata.insert(key.to_string(), value.to_string());
                    }
                }

                // Add instance ID and region to metadata
                if let Some(id) = instance.instance_id() {
                    metadata.insert("instance_id".to_string(), id.to_string());
                }
                metadata.insert("region".to_string(), self.region.clone());

                nodes.push(Node {
                    address: private_ip,
                    meta: metadata,
                });
            }
        }

        Ok(nodes)
    }
}
