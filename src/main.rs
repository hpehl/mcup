use anyhow::{bail, Result};
use app::build_app;
use glob::Pattern;

use crate::repo::{get_local_repo, process_local_repo};
use crate::version::VersionRange;

mod app;
mod artifact;
mod command;
mod filter;
mod group;
mod repo;
mod stats;
mod version;

fn main() -> Result<()> {
    let args = build_app()
        .mut_arg("artifacts", |arg| arg.validator(validate_artifacts))
        .mut_arg("versions", |arg| arg.validator(validate_versions))
        .get_matches();

    let local_repo = get_local_repo(&args)?;
    if local_repo.exists() {
        let stats = process_local_repo(local_repo.as_path(), &args);
        if atty::is(atty::Stream::Stdout) {
            println!();
            stats.summary();
        }
        Ok(())
    } else {
        bail!(
            "Local maven repository does not exist: '{}'",
            local_repo.display()
        )
    }
}

fn validate_artifacts(artifacts: &str) -> Result<(), String> {
    match Pattern::new(artifacts) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Illegal artifact pattern: {}", e.msg)),
    }
}

fn validate_versions(version: &str) -> Result<(), String> {
    match VersionRange::parse(version) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
