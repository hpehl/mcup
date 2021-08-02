use std::env;
use std::path::Path;
use std::process;

use anyhow::{Result, Context};
use clap_generate::generate_to;
use clap_generate::generators::{Bash, Elvish, Fish, PowerShell, Zsh};
use process::Command;

include!("src/app.rs");

const APP_NAME: &str = "mcup";

fn main() -> Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(out_dir.as_str());

    generate_shell_completions(out_dir)?;
    generate_man_page(out_dir)?;
    Ok(())
}

fn generate_shell_completions(out: &Path) -> Result<()> {
    let mut app = build_app();
    app.set_bin_name(APP_NAME);

    generate_to::<Bash, _, _>(&mut app, APP_NAME, out);
    generate_to::<Fish, _, _>(&mut app, APP_NAME, out);
    generate_to::<Zsh, _, _>(&mut app, APP_NAME, out);
    generate_to::<PowerShell, _, _>(&mut app, APP_NAME, out);
    generate_to::<Elvish, _, _>(&mut app, APP_NAME, out);
    Ok(())
}

fn generate_man_page(out: &Path) -> Result<()> {
    Command::new("asciidoctor").output().with_context(|| "Could not run 'asciidoctor' binary.")?;

    let cwd = env::current_dir()?;
    let adoc = cwd.join("doc").join(format!("{}.adoc", APP_NAME));
    let man = out.join(format!("{}.1", APP_NAME));
    process::Command::new("asciidoctor")
        .arg("--doctype")
        .arg("manpage")
        .arg("--backend")
        .arg("manpage")
        .arg(&adoc)
        .arg("--out-file")
        .arg(&man)
        .spawn()?
        .wait()
        .with_context(|| "'asciidoctor' failed")?;
    Ok(())
}
