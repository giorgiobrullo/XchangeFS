use xchangefs::{config, logger, networking};

#[tokio::main]
pub async fn main() -> () {
    logger::init_logger();
    let config = config::AppConfig::new().unwrap();
    tracing::debug!("Config loaded: {:?}", config);

    tracing::info!("Starting XchangeFS node...");

    // Start the networking module
    let network = networking::Network::new(xchangefs::networking::NetworkConfig { listen_addr: config.listen_addr, data_dir: config.data_dir });

    if let Err(e) = network.start().await {
        tracing::error!("Networking module failed: {:?}", e);
    }
}
