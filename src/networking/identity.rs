use std::{fs, error::Error, path::PathBuf};
use libp2p::identity;
use dirs::data_dir;
use tracing::info;

// Load the keypair from disk or generate a new one if it doesn't exist
pub fn load_or_generate_identity() -> Result<identity::Keypair, Box<dyn Error>> {
    let path = get_keypair_file_path()?;

    if path.exists() {
        // Load the keypair from file
        let keypair_bytes = fs::read(path)?;
        let keypair = identity::Keypair::from_protobuf_encoding(&keypair_bytes)?;
        info!("Loaded identity keypair from disk");
        Ok(keypair)
    } else {
        // Generate a new keypair and save it
        let keypair = identity::Keypair::generate_ed25519();
        let keypair_bytes = keypair.to_protobuf_encoding();
        info!("Generated new identity keypair");
        fs::write(path, keypair_bytes?)?;
        Ok(keypair)
    }
}

// Get the path to store the keypair, cross-platform
fn get_keypair_file_path() -> Result<PathBuf, Box<dyn Error>> {
    let mut path = data_dir().ok_or("Could not find data directory")?;
    path.push("XchangeFS"); // You can name this directory however you like
    fs::create_dir_all(&path)?; // Create the directory if it doesn't exist
    path.push("identity_keypair"); // The keypair file

    Ok(path)
}



