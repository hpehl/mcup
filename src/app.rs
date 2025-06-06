use clap::builder::styling::{AnsiColor, Effects};
use clap::builder::Styles;
use clap::{crate_name, crate_version, Arg, ArgAction, Command};

pub fn build_app() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .about("Command line tool to keep your local maven repository small and tidy.")
        .styles(Styles::styled()
            .header(AnsiColor::Green.on_default() | Effects::BOLD)
            .usage(AnsiColor::Green.on_default() | Effects::BOLD)
            .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
            .placeholder(AnsiColor::Cyan.on_default()))
        .propagate_version(true)
        .subcommand_required(true)
        .arg(Arg::new("groups")
            .short('g')
            .long("groups")
            .value_name("GROUPS")
            .display_order(1)
            .help("Selects artifacts based on the group ID. Subgroups are included by default."))
        .arg(Arg::new("artifacts")
            .short('a')
            .long("artifacts")
            .value_name("ARTIFACTS")
            .display_order(2)
            .help("Selects artifacts based on the artifact ID. Supports globbing like in 'maven-*-plugin'."))
        .arg(Arg::new("versions")
            .short('v')
            .long("versions")
            .value_name("VERSIONS")
            .display_order(3)
            .help("Selects artifacts based on version (ranges). Use '<n>..' to select the n most recent versions, '..<n>' to select the n oldest versions and '<version>' to select one specific version only."))
        .arg(Arg::new("local-repository")
            .short('l')
            .long("local-repository")
            .value_name("LOCAL_REPOSITORY")
            .help("Sets the location of the local maven repository. Respects the directory configured in '~/.m2/settings.xml'. Falls back to '~/.m2/repository', if nothing has been specified or configured."))
        .arg(Arg::new("releases")
            .short('r')
            .long("releases")
            .action(ArgAction::SetTrue)
            .help("Selects released artifacts only")
            .conflicts_with("snapshots"))
        .arg(Arg::new("snapshots")
            .short('s')
            .long("snapshots")
            .action(ArgAction::SetTrue)
            .help("Selects snapshot artifacts only")
            .conflicts_with("releases"))
        .subcommand(Command::new("keep")
            .about("Keeps the artifacts matched by the filters and removes the rest")
            .arg(Arg::new("dry-run")
                .short('d')
                .long("dry-run")
                .action(ArgAction::SetTrue)
                .help("Does not remove artifacts"))
            .arg(Arg::new("list")
                .long("list")
                .action(ArgAction::SetTrue)
                .help("Prints the full path to the artifacts that will be removed")))
        .subcommand(Command::new("rm")
            .about("Removes the artifacts matched by the filters and keeps the rest")
            .arg(Arg::new("dry-run")
                .short('d')
                .long("dry-run")
                .action(ArgAction::SetTrue)
                .help("Does not remove artifacts"))
            .arg(Arg::new("list")
                .long("list")
                .action(ArgAction::SetTrue)
                .help("Prints the full path to the artifacts that will be removed")))
        .subcommand(Command::new("du")
            .about("Analyzes the size of the artifacts selected by the filters")
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT")
                .default_value("ga")
                .help("Defines whether (g)roups, (a)rtifacts and (v)ersions are included in the usage summary")))
}
