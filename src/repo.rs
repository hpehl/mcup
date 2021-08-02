use std::ffi::OsStr;
use std::fs::{remove_dir_all, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use anyhow::{Context, Result};
use clap::ArgMatches;
use directories::BaseDirs;
use indicatif::{ProgressBar, ProgressStyle};
use multimap::MultiMap;
use quick_xml::events::Event;
use quick_xml::Reader;
use walkdir::{DirEntry, WalkDir};

use crate::artifact::Artifact;
use crate::command::Command;
use crate::command::Command::{Keep, Remove};
use crate::filter::Filter;
use crate::stats::Stats;
use crate::version::VersionRange::{Latest, Oldest};

const PROGRESS_BAR_THRESHOLD: usize = 5;

// Get the local repository in that order:
//   1. The value of the option `--local-repository`
//   2. The value of `<localRepository/>` in `~/.m2/settings.xml`
//   3. Fall back to `~/.m2/repository/`
// Whether the local repository really exists is the caller's responsibility
pub fn get_local_repo(args: &ArgMatches) -> Result<PathBuf> {
    match args.value_of("local-repository") {
        Some(path) => {
            // use --local-repository option
            Ok(Path::new(path).to_path_buf())
        }
        None => {
            // try ~/.m2/settings.xml
            let base_dirs = BaseDirs::new().with_context(|| "No home directory")?;
            let home = base_dirs.home_dir();
            let settings_xml = home.join(".m2/settings.xml");
            if settings_xml.exists() {
                let f =
                    File::open(settings_xml).with_context(|| "Unable to read ~/.settings.xml")?;
                let buf = BufReader::new(f);
                let mut reader = Reader::from_reader(buf);
                reader.trim_text(true);

                let mut buf = Vec::new();
                let mut txt = Vec::new();
                let local_repo: Option<String> = loop {
                    match reader.read_event(&mut buf) {
                        Ok(Event::Start(ref e)) => {
                            if e.name() == b"localRepository" {
                                break reader.read_text(b"localRepository", &mut txt).ok();
                            }
                        }
                        Ok(Event::Eof) => break None,
                        Err(_) => break None,
                        _ => {}
                    };
                    buf.clear();
                };
                match local_repo {
                    Some(value) => {
                        // use <localRepository/> value from ~/.m2/settings.xml
                        Ok(Path::new(&value).to_path_buf())
                    }
                    None => {
                        // no <localRepository/> tag, fall back to ~/.m2/repository
                        Ok(base_dirs.home_dir().join(".m2/repository"))
                    }
                }
            } else {
                // no ~/.m2/settings.xml, fall back to ~/.m2/repository
                Ok(base_dirs.home_dir().join(".m2/repository"))
            }
        }
    }
}

pub fn process_local_repo(local_repo: &Path, args: &ArgMatches) -> Stats {
    let command = Command::from(args);
    let filter = Filter::from(local_repo, args);
    let mut stats = Stats::start(args.is_present("dry-run"));

    // collect artifacts
    let removals = if let Some(group_filter) = &filter.group_filter {
        if filter.artifact_filter.is_none()
            && filter.version_range.is_none()
            && filter.release_type.is_none()
        {
            // groups only
            match command {
                Keep => {
                    // Remove everything that is not part of the specified (sub)groups
                    collect_artifacts(
                        local_repo,
                        |dir_entry| group_filter.no_subgroup_of(dir_entry),
                        |_| true,
                    )
                }
                Remove => {
                    // remove specified (sub)groups
                    collect_artifacts(
                        local_repo,
                        |dir_entry| group_filter.subgroup_of(dir_entry),
                        |artifact| group_filter.match_group_id(artifact),
                    )
                }
            }
        } else {
            // scope is the specified group
            // then apply the specified filters
            collect_artifacts(
                local_repo,
                |dir_entry| group_filter.subgroup_of(dir_entry),
                |artifact| {
                    group_filter.match_group_id(artifact) && filter.conjunction(artifact, &command)
                },
            )
        }
    } else {
        // no groups
        // apply the specified filters
        collect_artifacts(
            local_repo,
            |_| true,
            |artifact| filter.conjunction(artifact, &command),
        )
    };

    // filter version ranges
    let removals = if let Some(version_range) = filter.version_range {
        match version_range {
            Latest(_) | Oldest(_) => {
                let mut artifacts_by_ga: MultiMap<String, Artifact> = MultiMap::new();
                for artifact in removals {
                    let ga = format!("{}:{}", artifact.group_id, artifact.artifact_id);
                    artifacts_by_ga.insert(ga, artifact);
                }
                let mut collect_removals: Vec<Artifact> = Vec::new();
                for (_, artifacts_of_ga) in artifacts_by_ga.iter_all_mut() {
                    artifacts_of_ga.sort_by(|a, b| b.version.cmp(&a.version));
                    for artifact in command.removals(&version_range, artifacts_of_ga.as_slice()) {
                        collect_removals.push(artifact.clone());
                    }
                }
                collect_removals
            }
            _ => removals,
        }
    } else {
        removals
    };

    // remove artifacts
    stats.update(removals.as_slice());
    let dry_run = args.is_present("dry-run");
    let list = args.is_present("list");
    remove_artifacts(removals.as_slice(), dry_run, list, &mut stats);

    // done
    stats.finish();
    stats
}

fn collect_artifacts<P, Q>(
    local_repo: &Path,
    walk_predicate: P,
    artifact_predicate: Q,
) -> Vec<Artifact>
where
    P: FnMut(&DirEntry) -> bool,
    Q: Fn(&Artifact) -> bool,
{
    let mut artifacts: Vec<Artifact> = Vec::new();
    let progress_bar = ProgressBar::new_spinner()
        .with_prefix("Check artifacts")
        .with_style(
            ProgressStyle::default_spinner()
                .tick_chars("/|\\- ")
                .template("{spinner:.dim.bold} {prefix} {wide_msg}"),
        );
    progress_bar.enable_steady_tick(100);

    for dir_entry in WalkDir::new(local_repo)
        .min_depth(1)
        .into_iter()
        .filter_entry(walk_predicate)
        .filter_map(|e| e.ok())
    {
        if dir_entry.path().is_file()
            && dir_entry
                .path()
                .extension()
                .unwrap_or_else(|| OsStr::new(""))
                == "pom"
        {
            if let Ok(artifact) = Artifact::from(local_repo, dir_entry.path()) {
                progress_bar.set_message(artifact.to_string());
                if artifact_predicate(&artifact) {
                    artifacts.push(artifact);
                }
            }
        }
    }

    progress_bar.finish_and_clear();
    artifacts
}

fn remove_artifacts(artifacts: &[Artifact], dry_run: bool, list: bool, stats: &mut Stats) {
    let progress_bar = if !list && artifacts.len() > PROGRESS_BAR_THRESHOLD {
        Some(
            ProgressBar::new(artifacts.len() as u64)
                .with_prefix("Remove artifacts")
                .with_style(
                    ProgressStyle::default_bar()
                        .progress_chars("#>-")
                        .template("{prefix} [{wide_bar:.green/yellow}] {pos:>6}/{len:6}"),
                ),
        )
    } else {
        None
    };

    for artifact in artifacts {
        stats.bytes(artifact);
        if let Some(progress_bar) = &progress_bar {
            progress_bar.inc(1);
        }
        if list {
            let path = artifact.version_path.as_os_str();
            if let Some(str) = path.to_str() {
                println!("{}", str);
            }
        }
        if dry_run {
            if !list {
                thread::sleep(Duration::from_millis(5));
            }
        } else if let Err(error) = remove_dir_all(artifact.version_path.as_path()) {
            if let Some(path) = artifact.version_path.as_path().to_str() {
                stats.error(path.to_string(), error.to_string());
            }
        }
    }

    if let Some(progress_bar) = &progress_bar {
        progress_bar.finish_and_clear();
    }
}
