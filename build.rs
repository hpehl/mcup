use std::env;
use std::path::Path;

use anyhow::{Context, Result};
use clap_complete::generate_to;
use clap_complete::shells::{Bash, Elvish, Fish, PowerShell, Zsh};

include!("src/app.rs");

const APP_NAME: &str = "mcup";

fn main() -> Result<()> {
    generate_shell_completions()?;
    Ok(())
}

fn generate_shell_completions() -> Result<()> {
    let manifest_dir =
        env::var("CARGO_MANIFEST_DIR").with_context(|| "CARGO_MANIFEST_DIR not set")?;
    let manifest_dir = Path::new(manifest_dir.as_str()).join("completions/");
    let mut app = build_app();
    app.set_bin_name(APP_NAME);

    generate_to(Bash, &mut app, APP_NAME, &manifest_dir)?;
    generate_to(Fish, &mut app, APP_NAME, &manifest_dir)?;
    generate_to(Zsh, &mut app, APP_NAME, &manifest_dir)?;
    generate_to(PowerShell, &mut app, APP_NAME, &manifest_dir)?;
    generate_to(Elvish, &mut app, APP_NAME, &manifest_dir)?;
    Ok(())
}
