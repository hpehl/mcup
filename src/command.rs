use clap::ArgMatches;

use crate::command::Command::{Du, Keep, Remove};

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    // dry-run, list
    Keep(bool, bool),
    Remove(bool, bool),
    // groups, artifacts, versions
    Du(bool, bool, bool),
}

impl Command {
    pub fn from(args: &ArgMatches) -> Command {
        if args.subcommand_matches("keep").is_some() {
            let sub_args = args.subcommand_matches("keep").unwrap();
            Keep(sub_args.is_present("dry-run"), sub_args.is_present("list"))
        } else if args.subcommand_matches("rm").is_some() {
            let sub_args = args.subcommand_matches("rm").unwrap();
            Remove(sub_args.is_present("dry-run"), sub_args.is_present("list"))
        } else if args.subcommand_matches("du").is_some() {
            let sub_args = args.subcommand_matches("du").unwrap();
            let output = sub_args.value_of("output");
            let (groups, artifacts, versions) = if let Some(output) = output {
                (
                    output.contains('g'),
                    output.contains('a'),
                    output.contains('v'),
                )
            } else {
                (false, false, false)
            };
            Du(groups, artifacts, versions)
        } else {
            // Should not happen, since we use
            // AppSettings::SubcommandRequired
            panic!("No subcommand!")
        }
    }
}
