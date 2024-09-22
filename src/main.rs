use xchangefs::{config, logger, networking, filesystem};
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
    let fs = start_filesystem(&config)?;

    // Wait for the Ctrl+C shutdown signal
    wait_for_shutdown_signal().await;

    // Perform graceful shutdown of the network and filesystem modules
    shutdown_filesystem(fs)?;

    tracing::info!("XchangeFS node shut down gracefully.");
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

// Start the filesystem module
fn start_filesystem(config: &config::AppConfig) -> Result<filesystem::NoSpaceFs, Box<dyn Error>> {
    let mut fs = filesystem::NoSpaceFs::new(filesystem::FilesystemConfig {
        mount_path: config.mount_path.clone(),
    });

    fs.mount()?;
    tracing::info!("Filesystem module started successfully!");

    Ok(fs)
}

// Shutdown the filesystem module gracefully
fn shutdown_filesystem(mut fs: filesystem::NoSpaceFs) -> Result<(), Box<dyn Error>> {
    tracing::info!("Shutting down filesystem module...");
    fs.unmount()?; // Assuming an unmount method exists
    tracing::info!("Filesystem module shut down.");
    Ok(())
}

// Wait for the shutdown signal (Ctrl+C)
async fn wait_for_shutdown_signal() {
    match tokio::signal::ctrl_c().await {
        Ok(()) => tracing::info!("Received Ctrl-C signal, shutting down..."),
        Err(e) => tracing::error!("Failed to listen for shutdown signal: {:?}", e),
    }
}