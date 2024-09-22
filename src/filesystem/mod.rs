

use std::path::PathBuf;

use fuser::{Filesystem, MountOption, Session};
use tracing::{error, info};

// Configuration struct to hold the filesystem configuration
#[derive(Clone)]
pub struct FilesystemConfig {
    pub mount_path: PathBuf,
    // Additional configuration options can be added here in the future
}

impl FilesystemConfig {
    pub fn new(mount_path: PathBuf) -> Self {
        FilesystemConfig {
            mount_path,
            // Initialize other config options here
        }
    }
}

// Main struct representing the NoSpaceFs filesystem
pub struct NoSpaceFs {
    config: FilesystemConfig,
    session: Option<Session<NullFS>>,
}

impl NoSpaceFs {
    // Constructor to create a new instance of NoSpaceFs
    pub fn new(config: FilesystemConfig) -> Self {
        NoSpaceFs {
            config,
            session: None,
        }
    }

    // Method to mount the filesystem
    pub fn mount(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let config = self.config.clone(); // Clone the configuration for the closure

        let options = [
            MountOption::FSName("XchangeFS".to_string()),
            //MountOption::RO,  // Read-only mode for safety
            MountOption::AllowOther,
            MountOption::AutoUnmount,
        ];

        // Attempt to mount the filesystem
        Ok(match fuser::Session::new(NullFS, &config.mount_path, &options) {
            Ok(session) => {
                self.session = Some(session);
                info!("Filesystem mounted successfully at {}", config.mount_path.display())
            },
            Err(e) => {
                error!("Failed to mount filesystem: {}", e);
                return Err(e.into());
            },
        })
    }

    pub fn unmount(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.session.take().unwrap().unmount();
        Ok(())
    }
}

struct NullFS;

impl Filesystem for NullFS {}