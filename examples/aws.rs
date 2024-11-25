#[cfg(feature = "aws")]
use cloud_node_discovery::{Discovery, DiscoveryError};

#[cfg(feature = "aws")]
use tokio;

#[cfg(feature = "aws")]
#[tokio::main]
async fn main() -> Result<(), DiscoveryError> {
    let discovery = Discovery::new("aws", "region=us-east-1,tag_key=foo,tag_value=bar").await?;
    let nodes = discovery.discover().await.unwrap();
    println!("{:?}", nodes);
    Ok(())
}

// Add this fallback main function for when FFI is disabled
#[cfg(not(feature = "aws"))]
fn main() {
    println!("This example requires the 'aws' feature to be enabled.");
    println!("Please run with: cargo run --feature aws");
}