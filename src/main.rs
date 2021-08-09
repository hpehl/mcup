use anyhow::{bail, Result};
use clap::ArgMatches;
use glob::Pattern;

use app::build_app;

use crate::command::Command;
use crate::filter::Filter;
use crate::repo::Repository;
use crate::version::VersionRange;
use console::style;

mod app;
mod artifact;
mod command;
mod filter;
mod group;
mod repo;
mod version;

#[macro_use]
extern crate lazy_static;

fn main() -> Result<()> {
    let args = build_app()
        .mut_arg("artifacts", |arg| arg.validator(validate_artifacts))
        .mut_arg("versions", |arg| arg.validator(validate_versions))
        .get_matches();
    validate_command(&args)?;

    let mut local_repo = Repository::locate(&args)?;
    if local_repo.exists() {
        let command = Command::from(&args);
        let filter = Filter::from(&args, local_repo.path.as_path());
        let duration = local_repo.process(&command, &filter);
        if atty::is(atty::Stream::Stdout) {
            println!();
            command.summary(&local_repo, duration);
        }
        Ok(())
    } else {
        bail!(
            "Local maven repository does not exist: '{}'",
            local_repo.path.display()
        )
    }
}

// ------------------------------------------------------ validation

fn validate_command(args: &ArgMatches) -> Result<()> {
    if (args.subcommand_matches("keep").is_some() || args.subcommand_matches("rm").is_some())
        && !args.is_present("groups")
        && !args.is_present("artifacts")
        && !args.is_present("versions")
        && !args.is_present("snapshots")
        && !args.is_present("releases")
    {
        bail!(
            r#"Subcommand {} requires a filter, but one was not provided

USAGE:
    mcup [FLAGS] [OPTIONS] <SUBCOMMAND>

For more information try {}"#,
            style(format!("'{}'", args.subcommand_name().unwrap())).yellow(),
            style("--help").green()
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
