pub mod swarm;
pub mod identity;
pub mod kademlia; // Add a new module for Kademlia
pub mod address;  // Optional, if you decide to separate address parsing

use identity::load_or_generate_identity;
use tracing::info;
use std::error::Error;

// Define a configuration struct for the network module
pub struct NetworkConfig {
    pub listen_addr: Vec<String>,
}

pub struct Network {
    config: NetworkConfig,
}

impl Network {
    // Initialize the Network with configuration (Dependency Injection)
    pub fn new(config: NetworkConfig) -> Self {
        Self { config }
    }

    // Start the network using the injected configuration
    pub async fn start(&self) -> Result<(), Box<dyn Error>> {
        info!("Network module started!");
        let swarm = swarm::build_swarm(self.config.listen_addr.clone(), load_or_generate_identity()?).await?;
        swarm::run_swarm(swarm).await?;

        Ok(())
    }
}