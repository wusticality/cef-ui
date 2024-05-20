use anyhow::{anyhow, Result};
use cef_ui_util::{get_cef_artifacts_dir, get_cef_workspace_dir, AppBundleSettings, BuildCommand};
use clap::Parser;
use tracing::{level_filters::LevelFilter, subscriber::set_global_default, Level};
use tracing_log::LogTracer;
use tracing_subscriber::FmtSubscriber;

/// The examples that can be compiled.
const EXAMPLES: [&str; 2] = ["simple", "windowless"];

/// Command line arguments.
#[derive(Parser, Default)]
struct BuildArgs {
    /// Whether this is a release build.
    #[arg(long, default_value_t = String::from("dev"))]
    pub profile: String,

    /// Which example to build.
    #[arg(long, default_value_t = String::from("cef-ui-simple"))]
    pub example: String
}

fn main() -> Result<()> {
    // This routes log macros through tracing.
    LogTracer::init()?;

    // Setup the tracing subscriber globally.
    let subscriber = FmtSubscriber::builder()
        .with_max_level(LevelFilter::from_level(Level::INFO))
        .finish();

    set_global_default(subscriber)?;

    let args = BuildArgs::parse();
    let workspace_dir = get_cef_workspace_dir()?;

    // Make sure we're compiling a valid example.
    if !EXAMPLES.contains(&args.example.as_str()) {
        return Err(anyhow!("Unknown example: {}", args.example));
    }

    let example = format!("cef-ui-{}", args.example);

    // Build the main executable.
    BuildCommand {
        binary:  example.clone(),
        profile: args.profile.to_string()
    }
    .run()?;

    // If on macOS, we need to do some extra work.
    if cfg!(target_os = "macos") {
        let helper = format!("{}-helper", example);

        // Build the helper executable.
        BuildCommand {
            binary:  helper.clone(),
            profile: args.profile.to_string()
        }
        .run()?;

        // Build the app bundle.
        AppBundleSettings {
            profile:         args.profile.to_string(),
            artifacts_dir:   get_cef_artifacts_dir()?,
            app_name:        example.clone(),
            main_exe_name:   example,
            helper_exe_name: helper,
            resources_dir:   workspace_dir.join("resources/macos"),
            org_name:        String::from("hytopia")
        }
        .run()?;
    }

    Ok(())
}
