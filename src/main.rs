use xchangefs::{config, logger, networking};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logger
    logger::init_logger();

    // Load configuration
    let config = load_config()?;

    tracing::info!("Starting XchangeFS node...");

    // Start the networking and filesystem modules
    let _network = start_networking(&config).await?;

    // Wait for the Ctrl+C shutdown signal
    wait_for_shutdown_signal().await;

    tracing::info!("XchangeFS node shut down gracefully. See you soon!");
    Ok(())
}

// Load the application configuration
fn load_config() -> Result<config::AppConfig, Box<dyn Error>> {
    let config = config::AppConfig::new()?;
    tracing::debug!("Config loaded: {:?}", config);
    Ok(config)
}

// Start the networking module
async fn start_networking(config: &config::AppConfig) -> Result<networking::Network, Box<dyn Error>> {
    let network = networking::Network::new(networking::NetworkConfig {
        listen_addr: config.listen_addr.clone(),
        data_dir: config.data_dir.clone(),
    });

    network.start().await?;
    tracing::info!("Networking module started successfully!");

    Ok(network)
}


// Wait for the shutdown signal (Ctrl+C)
async fn wait_for_shutdown_signal() {
    match tokio::signal::ctrl_c().await {
        Ok(()) => tracing::info!("Received Ctrl-C signal, shutting down..."),
        Err(e) => tracing::error!("Failed to listen for shutdown signal: {:?}", e),
    }
}