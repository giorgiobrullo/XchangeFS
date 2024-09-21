use std::error::Error;
use std::io::Write;
use std::path::PathBuf;
use serial_test::serial;
use temp_env::async_with_vars;
use tempfile::NamedTempFile;
use xchangefs::config::AppConfig;

#[tokio::test]
#[serial]
async fn test_load_config_from_custom_file() -> Result<(), Box<dyn Error>> {
    // Use tempfile to create a temporary config.toml file
    let temp_file = NamedTempFile::new()?;
    let temp_path = temp_file.path().with_extension(".toml").to_path_buf();

    let persisted_path = temp_path.clone();
    let mut persisted_file = temp_file.persist(persisted_path)?;

    // Write config content to the file asynchronously
    persisted_file.write_all(
        r#"
        data_dir = "/custom/data/dir"
        listen_addr = ["127.0.0.1:8080", "192.168.1.1:8080"]
        idle_timeout_secs = 600
    "#
        .as_bytes(),
    )?;
    persisted_file.sync_all()?;

    // Temporarily set the CONFIG environment variable to point to the temp file
    async_with_vars([("XCHANGEFS__CONFIG", Some(temp_path.to_str().unwrap()))], async {
        let config = AppConfig::new()?;
        
        assert_eq!(config.data_dir, PathBuf::from("/custom/data/dir"));
        assert_eq!(
            config.listen_addr,
            vec![
                "127.0.0.1:8080".to_string(),
                "192.168.1.1:8080".to_string()
            ]
        );
        assert_eq!(config.idle_timeout_secs, 600);

        Ok::<(), Box<dyn Error>>(())
    }).await
}

#[tokio::test]
#[serial]
async fn test_load_config_from_env() -> Result<(), Box<dyn Error>> {
    async_with_vars(
        [
            ("XCHANGEFS__DATA_DIR", Some("/env/data/dir")),
            ("XCHANGEFS__LISTEN_ADDR", Some("127.0.0.1:8081,192.168.1.1:8081")),
            ("XCHANGEFS__IDLE_TIMEOUT_SECS", Some("900")),
        ],
        async {
            let config = AppConfig::new()?;
    
            assert_eq!(config.data_dir, PathBuf::from("/env/data/dir"));
            assert_eq!(
                config.listen_addr,
                vec!["127.0.0.1:8081".to_string(), "192.168.1.1:8081".to_string()]
            );
            assert_eq!(config.idle_timeout_secs, 900);
    
            Ok::<(), Box<dyn Error>>(())
        }
    ).await
}