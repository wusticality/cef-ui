use anyhow::Result;
use std::{env::current_dir, fs::remove_dir_all, process::Command};
use tracing::{level_filters::LevelFilter, subscriber::set_global_default, Level};
use tracing_log::LogTracer;
use tracing_subscriber::FmtSubscriber;

/// This does the following:
/// * Runs cargo clean.
/// * Removes intermediate artifacts.
pub fn clean_all() -> Result<()> {
    // This routes log macros through tracing.
    LogTracer::init()?;

    // Setup the tracing subscriber globally.
    let subscriber = FmtSubscriber::builder()
        .with_max_level(LevelFilter::from_level(Level::INFO))
        .finish();

    set_global_default(subscriber)?;

    Command::new("cargo")
        .args(&["clean"])
        .output()?;

    let artifacts_dir = current_dir()?.join("artifacts");

    // Remove the artifacts directory.
    if artifacts_dir.exists() {
        remove_dir_all(&artifacts_dir)?;
    }

    Ok(())
}
