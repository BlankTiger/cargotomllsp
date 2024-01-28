use std::{fs::OpenOptions, path::Path};

use anyhow::Result;
use once_cell::sync::Lazy;
use tracing_subscriber::EnvFilter;

static LOG_FILE: Lazy<String> = Lazy::new(|| {
    let home_path = std::env!("HOME");
    home_path.to_string() + "/.local/state/cargotomllsp.log"
});

fn get_log_file() -> Result<std::fs::File> {
    let log_path = &LOG_FILE.to_string();
    let log_path = Path::new(log_path);
    println!("{:?}", log_path);
    if !std::path::Path::exists(log_path) {
        std::fs::File::create(log_path)?;
    }
    let file = OpenOptions::new().append(true).open(log_path)?;
    Ok(file)
}

pub fn setup_logging() -> Result<()> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }

    let log_file = get_log_file()?;
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(log_file)
        .init();

    Ok(())
}
