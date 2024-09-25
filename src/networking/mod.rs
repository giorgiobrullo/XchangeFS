pub mod address;
pub mod identity;
pub mod swarm;
pub mod behavior;
//pub mod protocol;

use identity::load_or_generate_identity;
use std::{error::Error, path::PathBuf};
use tracing::{error, info};

// Define a configuration struct for the network module
pub struct NetworkConfig {
    pub listen_addr: Vec<String>,
    pub data_dir: PathBuf,
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
        let swarm = swarm::build_swarm(
            self.config.listen_addr.clone(),
            load_or_generate_identity(self.config.data_dir.clone())?,
        )
        .await?;

        tokio::spawn(async move {
            if let Err(e) = swarm::run_swarm(swarm).await {
            error!("Error running swarm: {:?}", e);
            }
        });

        Ok(())
    }
}
