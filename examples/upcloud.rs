#[cfg(feature = "upcloud")]
use cloud_node_discovery::{Discovery, DiscoveryError};

#[cfg(feature = "upcloud")]
use tokio;

#[cfg(feature = "upcloud")]
#[tokio::main]
async fn main() -> Result<(), DiscoveryError> {
    let discovery = Discovery::new("upcloud", "zone=fi-hel1,label_key=env,label_value=prod").await?;
    let nodes = discovery.discover().await.unwrap();
    println!("{:?}", nodes);
    Ok(())
}

// Add this fallback main function for when FFI is disabled
#[cfg(not(feature = "upcloud"))]
fn main() {
    println!("This example requires the 'aws' feature to be enabled.");
    println!("Please run with: cargo run --feature aws");
}