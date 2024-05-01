use anyhow::Result;
use cef_ui_util::{get_build_rs_workspace_dir, link_cef};

fn main() -> Result<()> {
    let workspace_dir = get_build_rs_workspace_dir()?;
    let artifacts_dir = workspace_dir.join("artifacts");

    println!("cargo:warning=artifact_dir: {:?}", artifacts_dir);

    link_cef(&artifacts_dir)?;

    Ok(())
}
