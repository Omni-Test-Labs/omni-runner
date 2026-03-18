use tracing_subscriber::{EnvFilter, fmt};
use tracing_subscriber::prelude::*;

pub fn init_logging() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| {
                    EnvFilter::new("omni_runner=debug,info")
                })
        )
        .with(fmt::layer())
        .init();
}
