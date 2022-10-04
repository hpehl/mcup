use std::time::Duration;

use bit_vec::BitVec;
use clap::ArgMatches;
use console::{pad_str, Alignment, Style};
use indicatif::{DecimalBytes, HumanDuration};

use crate::artifact::Artifact;
use crate::command::Command::{Du, Keep, Remove};
use crate::group::Group;
use crate::repo::Repository;
use crate::version::Version;

// ------------------------------------------------------ command

#[derive(Clone, Debug, Eq, PartialEq)]
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
            Keep(sub_args.get_flag("dry-run"), sub_args.get_flag("list"))
        } else if args.subcommand_matches("rm").is_some() {
            let sub_args = args.subcommand_matches("rm").unwrap();
            Remove(sub_args.get_flag("dry-run"), sub_args.get_flag("list"))
        } else if args.subcommand_matches("du").is_some() {
            let sub_args = args.subcommand_matches("du").unwrap();
            let output = sub_args.get_one::<String>("output");
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

    pub fn summary(&self, repository: &Repository, duration: Duration) {
        match self {
            Keep(dry_run, _) | Remove(dry_run, _) => rm_summary(repository, duration, *dry_run),
            Du(groups, artifacts, versions) => {
                du_summary(repository, (*groups, *artifacts, *versions))
            }
        }
    }
}

// ------------------------------------------------------ common styles

struct Styles {
    dim: Style,
    normal: Style,
    bold: Style,
    groups: Style,
    artifacts: Style,
    versions: Style,
    summary: Style,
    dry_run: Style,
}

lazy_static! {
    static ref STYLES: Styles = Styles {
        dim: Style::new().dim(),
        normal: Style::new().for_stdout(),
        bold: Style::new().magenta().bold(),
        groups: Style::new().cyan(),
        artifacts: Style::new().green(),
        versions: Style::new().yellow(),
        summary: Style::new().green(),
        dry_run: Style::new().yellow(),
    };
}

// ------------------------------------------------------ rm / keep

fn rm_summary(repository: &Repository, duration: Duration, dry_run: bool) {
    let bytes = DecimalBytes(repository.bytes);
    let duration = HumanDuration(duration);

    println!(
        "The operation {}",
        if dry_run {
            STYLES.dry_run.apply_to("would affect ")
        } else {
            STYLES.normal.apply_to("affects")
        }
    );
    println!();
    println!(
        "    {} groups,",
        STYLES.summary.apply_to(repository.groups.len())
    );
    println!(
        "    {} artifacts and",
        STYLES.summary.apply_to(repository.artifacts)
    );
    println!(
        "    {} versions",
        STYLES.summary.apply_to(repository.versions)
    );
    println!();
    println!(
        "The operation took {} and {}released {}.",
        STYLES.summary.apply_to(duration),
        if dry_run {
            STYLES.dry_run.apply_to("would have ")
        } else {
            STYLES.normal.apply_to("")
        },
        STYLES.summary.apply_to(bytes)
    );
    if dry_run {
        println!(
            "Since you've used {} no artifacts have been removed.",
            STYLES.dry_run.apply_to("--dry-run")
        );
    }
}

// ------------------------------------------------------ du

const SIZE_COLUMN: usize = 10;
const TEXT_COLUMN: usize = 64;
const GROUPS_COLUMN: usize = 13;
const ARTIFACTS_COLUMN: usize = 16;
const VERSIONS_COLUMN: usize = 33;
const COUNT_WIDTH: usize = 4;

fn du_summary(repository: &Repository, gav: (bool, bool, bool)) {
    let (groups, artifacts, versions) = gav;
    let mut bits = BitVec::new();
    bits.push(groups);
    bits.push(artifacts);
    bits.push(versions);
    let hierarchy = bits.iter().filter(|b| *b).count() > 1;

    header(repository);
    if !repository.is_empty() {
        post_header();
    }
    body(repository, gav, hierarchy);
    if !repository.is_empty() {
        footer(repository, "┢", "╈", "┪");
    } else {
        footer(repository, "┣", "╋", "┫");
    }
}

// ------------------------------------------------------ header, body, footer

fn header(repository: &Repository) {
    let path = repository.path.display().to_string();

    dim("┏");
    line("━", SIZE_COLUMN);
    dim("┳");
    line("━", TEXT_COLUMN);
    dim("┓");
    println!();

    dim("┃");
    size_pad(repository.bytes, &STYLES.bold);
    dim("┃");
    text_pad(path.as_str(), TEXT_COLUMN, &STYLES.bold);
    dim("┃");
    println!();
}

fn post_header() {
    dim("┡");
    line("━", SIZE_COLUMN);
    dim("╇");
    line("━", TEXT_COLUMN);
    dim("┩");
    println!();
}

fn body(repository: &Repository, gav: (bool, bool, bool), hierarchy: bool) {
    let (groups, artifacts, versions) = gav;

    // groups
    let mut sorted_groups: Vec<&Group> = repository.groups.values().collect();
    sorted_groups.sort_by(|a, b| b.bytes.cmp(&a.bytes));
    for (group_index, group) in sorted_groups.iter().enumerate() {
        let last_group = group_index == sorted_groups.len() - 1;
        if groups {
            size_and_text(group.bytes, group.id.as_str(), &STYLES.groups);
        }

        // artifacts
        let mut sorted_artifacts: Vec<&Artifact> = group.artifacts.values().collect();
        sorted_artifacts.sort_by(|a, b| b.bytes.cmp(&a.bytes));
        for (artifact_index, artifact) in sorted_artifacts.iter().enumerate() {
            let last_artifact = artifact_index == sorted_artifacts.len() - 1;
            if artifacts {
                let name = if groups {
                    artifact.id.clone()
                } else {
                    format!("{}:{}", group.id.clone(), artifact.id.clone())
                };
                if hierarchy && groups {
                    size_and_text_1(
                        artifact.bytes,
                        name.as_str(),
                        &STYLES.artifacts,
                        last_artifact,
                    );
                } else {
                    size_and_text(artifact.bytes, name.as_str(), &STYLES.artifacts);
                }
            }

            // versions
            let mut sorted_versions: Vec<&Version> = artifact.versions.values().collect();
            sorted_versions.reverse();
            for (version_index, version) in sorted_versions.iter().enumerate() {
                let last_version = version_index == sorted_versions.len() - 1;
                if versions {
                    let name = if groups {
                        if artifacts {
                            version.to_string().clone()
                        } else {
                            format!("{}:{}", artifact.id.clone(), version.to_string().clone())
                        }
                    } else if artifacts {
                        version.to_string().clone()
                    } else {
                        format!("{}:{}:{}", group.id.clone(), artifact.id.clone(), version)
                    };
                    if hierarchy {
                        if groups && artifacts {
                            size_and_text_2(
                                version.bytes,
                                name.as_str(),
                                &STYLES.versions,
                                (last_artifact, last_version),
                            );
                        } else if groups || artifacts {
                            size_and_text_1(
                                version.bytes,
                                name.as_str(),
                                &STYLES.versions,
                                if groups {
                                    last_artifact && last_version
                                } else {
                                    last_version
                                },
                            );
                        }
                    } else {
                        size_and_text(version.bytes, name.as_str(), &STYLES.versions);
                    }
                }
            }
        }
        if !last_group && (artifacts || versions) {
            separator();
        }
    }
}

fn footer(
    repository: &Repository,
    vertical_left: &'static str,
    cross: &'static str,
    right_vertical: &'static str,
) {
    let groups = format!("{}", repository.groups.len());
    let artifacts = format!("{}", repository.artifacts);
    let versions = format!("{}", repository.versions);

    dim(vertical_left);
    line("━", SIZE_COLUMN);
    dim(cross);
    line("━", GROUPS_COLUMN);
    dim("┳");
    line("━", ARTIFACTS_COLUMN);
    dim("┳");
    line("━", VERSIONS_COLUMN);
    dim(right_vertical);
    println!();

    dim("┃");
    size_pad(repository.bytes, &STYLES.bold);
    dim("┃");
    println!(
        " {} {} {} {} {} {} {} {} {}",
        STYLES.bold.apply_to(pad_str(
            groups.as_str(),
            COUNT_WIDTH,
            Alignment::Right,
            None,
        )),
        STYLES.groups.apply_to("groups"),
        STYLES.dim.apply_to("┃"),
        STYLES.bold.apply_to(pad_str(
            artifacts.as_str(),
            COUNT_WIDTH,
            Alignment::Right,
            None,
        )),
        STYLES.artifacts.apply_to("artifacts"),
        STYLES.dim.apply_to("┃"),
        STYLES.bold.apply_to(pad_str(
            versions.as_str(),
            COUNT_WIDTH,
            Alignment::Right,
            None,
        )),
        STYLES.versions.apply_to(pad_str(
            "versions",
            VERSIONS_COLUMN - COUNT_WIDTH - 3,
            Alignment::Left,
            None,
        )),
        STYLES.dim.apply_to("┃"),
    );

    dim("┗");
    line("━", SIZE_COLUMN);
    dim("┻");
    line("━", GROUPS_COLUMN);
    dim("┻");
    line("━", ARTIFACTS_COLUMN);
    dim("┻");
    line("━", VERSIONS_COLUMN);
    dim("┛");
    println!();
}

// ------------------------------------------------------ helper functions

fn size_and_text(size: u64, text: &str, style: &Style) {
    dim("│");
    size_pad(size, style);
    dim("│");
    text_pad(text, TEXT_COLUMN, style);
    dim("│");
    println!();
}

fn size_and_text_1(size: u64, text: &str, style: &Style, last: bool) {
    dim("│");
    size_pad(size, style);
    dim("│");
    if last {
        dim(" └──");
    } else {
        dim(" ├──");
    }
    text_pad(text, TEXT_COLUMN - 4, style);
    dim("│");
    println!()
}

fn size_and_text_2(size: u64, text: &str, style: &Style, last: (bool, bool)) {
    dim("│");
    size_pad(size, style);
    dim("│");
    match last {
        (false, false) => dim(" │   ├──"),
        (false, true) => dim(" │   └──"),
        (true, false) => dim("     ├──"),
        (true, true) => dim("     └──"),
    }
    text_pad(text, TEXT_COLUMN - 8, style);
    dim("│");
    println!()
}

fn size_pad(size: u64, style: &Style) {
    let size = DecimalBytes(size).to_string();
    print!(
        " {} ",
        style.apply_to(pad_str(
            size.as_str(),
            SIZE_COLUMN - 2,
            Alignment::Right,
            None,
        )),
    );
}

fn text_pad(text: &str, length: usize, style: &Style) {
    print!(
        " {} ",
        style.apply_to(pad_str(text, length - 2, Alignment::Left, Some("...")))
    );
}

fn separator() {
    dim("├");
    line("─", SIZE_COLUMN);
    dim("┼");
    line("─", TEXT_COLUMN);
    dim("┤");
    println!();
}

fn line(symbol: &str, length: usize) {
    for _ in 0..length {
        dim(symbol);
    }
}

#[inline]
fn dim(text: &str) {
    print!("{}", STYLES.dim.apply_to(text));
}
