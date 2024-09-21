use tracing_subscriber::EnvFilter;

pub fn init_logger() {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("debug"))
        .finish();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_logger() {
        // This test simply ensures that the logger can be initialized without panicking.
        init_logger();
    }

    // TODO: Add more tests for the logger module
}
