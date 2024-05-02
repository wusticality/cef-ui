use anyhow::Result;
use cef_ui_util::{get_tool_workspace_dir, CleanCommand};
use tracing::{level_filters::LevelFilter, subscriber::set_global_default, Level};
use tracing_log::LogTracer;
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()> {
    // This routes log macros through tracing.
    LogTracer::init()?;

    // Setup the tracing subscriber globally.
    let subscriber = FmtSubscriber::builder()
        .with_max_level(LevelFilter::from_level(Level::INFO))
        .finish();

    set_global_default(subscriber)?;

    let workspace_dir = get_tool_workspace_dir()?;
    let artifacts_dir = workspace_dir.join("artifacts");

    CleanCommand { artifacts_dir }.run()?;

    Ok(())
}
