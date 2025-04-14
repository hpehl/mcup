#[macro_use]
extern crate lazy_static;

mod app;
mod artifact;
mod command;
mod filter;
mod group;
mod repo;
mod version;

use anyhow::{bail, Result};
use clap::ArgMatches;
use glob::Pattern;
use std::io::{stdout, IsTerminal};

use app::build_app;

use crate::command::Command;
use crate::filter::Filter;
use crate::repo::Repository;
use crate::version::VersionRange;
use console::style;

fn main() -> Result<()> {
    let args = build_app()
        .mut_arg("artifacts", |arg| arg.value_parser(parse_artifacts))
        .mut_arg("versions", |arg| arg.value_parser(parse_versions))
        .get_matches();
    validate_command(&args)?;

    let mut local_repo = Repository::locate(&args)?;
    if local_repo.exists() {
        let command = Command::from(&args);
        let filter = Filter::from(&args, local_repo.path.as_path());
        let duration = local_repo.process(&command, &filter);
        if stdout().is_terminal() {
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
        && !args.get_flag("groups")
        && !args.get_flag("artifacts")
        && !args.get_flag("versions")
        && !args.get_flag("snapshots")
        && !args.get_flag("releases")
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

fn parse_artifacts(artifacts: &str) -> Result<Pattern, String> {
    match Pattern::new(artifacts) {
        Ok(p) => Ok(p),
        Err(e) => Err(format!("Illegal artifact pattern: {}", e.msg)),
    }
}

fn parse_versions(version: &str) -> Result<VersionRange, String> {
    match VersionRange::parse(version) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}
