use anyhow::{bail, Result};
use clap::ArgMatches;
use glob::Pattern;

use app::build_app;

use crate::command::Command;
use crate::filter::Filter;
use crate::repo::Repository;
use crate::stats::{du, summary};
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
    validate_command(&args)?;

    let mut local_repo = Repository::locate(&args)?;
    if local_repo.exists() {
        let command = Command::from(&args);
        let filter = Filter::from(local_repo.path.as_path(), &args);
        let duration = local_repo.process(&command, &filter);
        if atty::is(atty::Stream::Stdout) {
            println!();
            match command {
                Command::Keep(dry_run, _) | Command::Remove(dry_run, _) => {
                    summary(&local_repo, duration, dry_run);
                }
                Command::Du(groups, artifacts, versions) => {
                    du(&local_repo, groups, artifacts, versions);
                }
            }
        }
        Ok(())
    } else {
        bail!(
            "Local maven repository does not exist: '{}'",
            local_repo.path.display()
        )
    }
}

fn validate_command(args: &ArgMatches) -> Result<()> {
    if (args.subcommand_matches("keep").is_some() || args.subcommand_matches("rm").is_some())
        && !args.is_present("groups")
        && !args.is_present("artifacts")
        && !args.is_present("versions")
        && !args.is_present("snapshots")
        && !args.is_present("releases")
    {
        bail!(
            "For subcommand '{}' at least one filter is required.",
            args.subcommand_name().unwrap()
        )
    }
    Ok(())
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
