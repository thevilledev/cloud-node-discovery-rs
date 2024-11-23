use cloud_node_discovery::{Discovery, DiscoveryError};
use tokio;

#[tokio::main]
async fn main() -> Result<(), DiscoveryError> {
    let discovery = Discovery::new("aws", "region=us-east-1,tag_key=foo,tag_value=bar").await?;
    let nodes = discovery.discover().await.unwrap();
    println!("{:?}", nodes);
    Ok(())
}
