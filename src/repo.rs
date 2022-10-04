use std::collections::{BTreeMap, HashSet};
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::fs::{read_to_string, remove_dir, remove_dir_all};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use clap::ArgMatches;
use directories::BaseDirs;
use indicatif::{ProgressBar, ProgressStyle};
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::Reader;
use walkdir::{DirEntry, WalkDir};

use crate::artifact::Artifact;
use crate::command::Command;
use crate::command::Command::{Du, Keep, Remove};
use crate::filter::Filter;
use crate::group::Group;
use crate::version::{Version, VersionRange};

const PROGRESS_BAR_THRESHOLD: usize = 5;

// ------------------------------------------------------ repo

#[derive(Debug)]
pub struct Repository {
    pub path: PathBuf,
    pub groups: BTreeMap<String, Group>,
    pub artifacts: usize,
    pub versions: usize,
    pub bytes: u64,
}

impl Repository {
    // Locate the local repository in that order:
    //   1. The value of the option `--local-repository`
    //   2. The value of `<localRepository/>` in `~/.m2/settings.xml`
    //   3. Fall back to `~/.m2/repository/`
    // Whether the local repository really exists is the caller's responsibility
    pub fn locate(args: &ArgMatches) -> Result<Repository> {
        match args.get_one::<String>("local-repository") {
            Some(path) => {
                // use --local-repository option
                Ok(Repository::new(Path::new(path).to_path_buf()))
            }
            None => {
                // try ~/.m2/settings.xml
                let base_dirs = BaseDirs::new().with_context(|| "No home directory")?;
                let home = base_dirs.home_dir();
                let settings_xml = home.join(".m2/settings.xml");
                if settings_xml.exists() {
                    let string = read_to_string(settings_xml)
                        .with_context(|| "Unable to read ~/.settings.xml")?;
                    let mut reader = Reader::from_str(string.as_str());
                    reader.trim_text(true);

                    let local_repo = loop {
                        match reader.read_event() {
                            Ok(Event::Start(ref e)) => {
                                if e.name().as_ref() == b"localRepository" {
                                    break reader
                                        .read_text(QName(b"localRepository"))
                                        .map(String::from)
                                        .ok();
                                }
                            }
                            Ok(Event::Eof) => break None,
                            Err(_) => break None,
                            _ => {}
                        };
                    };

                    match local_repo {
                        Some(value) => {
                            // use <localRepository/> value from ~/.m2/settings.xml
                            Ok(Repository::new(Path::new(&value).to_path_buf()))
                        }
                        None => {
                            // no <localRepository/> tag, fall back to ~/.m2/repository
                            Ok(Repository::new(base_dirs.home_dir().join(".m2/repository")))
                        }
                    }
                } else {
                    // no ~/.m2/settings.xml, fall back to ~/.m2/repository
                    Ok(Repository::new(base_dirs.home_dir().join(".m2/repository")))
                }
            }
        }
    }

    fn new(path: PathBuf) -> Repository {
        Repository {
            path,
            groups: BTreeMap::new(),
            artifacts: 0,
            versions: 0,
            bytes: 0,
        }
    }

    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }

    pub fn process(&mut self, command: &Command, filter: &Filter) -> Duration {
        let now = Instant::now();

        // collect GAVs
        let gavs = if let Some(ref group_filter) = filter.group_filter {
            if filter.artifact_filter.is_none()
                && filter.version_range.is_none()
                && filter.release_type.is_none()
            {
                // groups only
                match command {
                    Keep(_, _) => {
                        // Remove everything that is not part of the specified (sub)groups
                        self.collect(|dir_entry| group_filter.no_subgroup_of(dir_entry), |_| true)
                    }
                    Remove(_, _) | Du(_, _, _) => {
                        // remove or analyze specified (sub)groups
                        self.collect(
                            |dir_entry| group_filter.subgroup_of(dir_entry),
                            |gav| group_filter.match_group_id(gav.group.id.as_str()),
                        )
                    }
                }
            } else {
                // scope is the specified group
                // then apply the specified filters
                self.collect(
                    |dir_entry| group_filter.subgroup_of(dir_entry),
                    |gav| {
                        group_filter.match_group_id(gav.group.id.as_str())
                            && filter.conjunction(gav, command)
                    },
                )
            }
        } else {
            // no groups
            // apply the specified filters
            self.collect(|_| true, |artifact| filter.conjunction(artifact, command))
        };

        // add GAVs to repo
        self.add_all(gavs);

        // filter version ranges
        if let Some(ref version_range) = filter.version_range {
            self.remove_version_range(version_range, command);
        }

        // sum up bytes & counters
        self.compute();

        // remove versions
        if let Keep(dry_run, list) | Remove(dry_run, list) = command {
            self.remove_versions(*dry_run, *list);
            if !(*dry_run) {
                self.prune_empty_directories();
            }
        }

        // done
        now.elapsed()
    }

    fn collect<P, Q>(&mut self, walk_predicate: P, gav_predicate: Q) -> Vec<Gav>
    where
        P: FnMut(&DirEntry) -> bool,
        Q: Fn(&Gav) -> bool,
    {
        let mut gavs: Vec<Gav> = Vec::new();
        let progress_bar = ProgressBar::new_spinner()
            .with_prefix("Check artifacts")
            .with_style(
                ProgressStyle::default_spinner()
                    .tick_chars("/|\\- ")
                    .template("{spinner:.dim.bold} {prefix} {wide_msg}")
                    .unwrap(),
            );
        progress_bar.enable_steady_tick(Duration::from_millis(100));

        for dir_entry in WalkDir::new(self.path.as_path())
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
                if let Ok(gav) = self.gav(dir_entry.path()) {
                    progress_bar.set_message(gav.to_string());
                    if gav_predicate(&gav) {
                        gavs.push(gav);
                    }
                }
            }
        }

        progress_bar.finish_and_clear();
        gavs
    }

    fn gav(&mut self, pom: &Path) -> Result<Gav> {
        let version_path = pom.parent().with_context(|| "No version path")?;
        let version = Version::from_path(version_path)?;

        let artifact_path = version_path.parent().with_context(|| "No artifact path")?;
        let artifact_id = artifact_path
            .file_name()
            .with_context(|| "No artifact ID")?
            .to_str()
            .with_context(|| "No artifact ID")?;
        let artifact = Artifact::new(artifact_id, artifact_path);

        let group_path = artifact_path.parent().with_context(|| "No group path")?;
        let group_id: String = group_path
            .strip_prefix(self.path.as_path())?
            .components()
            .map(|c| c.as_os_str().to_str().unwrap_or(""))
            .collect::<Vec<&str>>()
            .join(".");
        let group = Group::new(group_id.as_str(), group_path);

        Ok(Gav {
            group,
            artifact,
            version,
        })
    }

    fn add_all(&mut self, gavs: Vec<Gav>) {
        for gav in gavs {
            let group = self.groups.entry(gav.group.id.clone()).or_insert(gav.group);
            let artifact = group
                .artifacts
                .entry(gav.artifact.id.clone())
                .or_insert(gav.artifact);
            artifact
                .versions
                .entry(gav.version.clone())
                .or_insert(gav.version);
        }
    }

    fn remove_version_range(&mut self, version_range: &VersionRange, command: &Command) {
        for group in self.groups.values_mut() {
            for artifact in group.artifacts.values_mut() {
                let mut artifact_versions: Vec<Version> =
                    artifact.versions.keys().cloned().collect();
                // Don't forget to reverse!
                // Versions are sorted from lowest to highest in the BTreeMap.
                // For the version range to work we need highest to lowest.
                artifact_versions.reverse();
                let selection = version_range.select(artifact_versions.as_slice());
                let mut selection_set = HashSet::new();
                for version in selection {
                    selection_set.insert(version);
                }
                // The selection is just a selection based on the version range.
                // Choose the right method depending on the command, but keep in
                // mind that the repository should only contain those artifacts
                // that should be removed.
                //    keep      User wants to keep the selected artifacts
                //              => remove them from the repo
                //    rm / du   User wants to remove / analyze the artifacts
                //              => retain them in the repo
                match command {
                    Keep(_, _) => {
                        for k in selection_set {
                            artifact.versions.remove(k);
                        }
                    }
                    Remove(_, _) | Du(_, _, _) => {
                        artifact.versions.retain(|k, _| selection_set.contains(k));
                    }
                }
            }
        }
    }

    fn compute(&mut self) {
        let mut artifacts: usize = 0;
        let mut versions: usize = 0;
        let mut repo_bytes: u64 = 0;

        for group in self.groups.values_mut() {
            let mut group_bytes: u64 = 0;
            for artifact in group.artifacts.values_mut() {
                let mut artifact_bytes: u64 = 0;
                for version in artifact.versions.values_mut() {
                    let mut version_bytes: u64 = 0;
                    if let Ok(read_dir) = version.path.read_dir() {
                        for dir_entry in read_dir.flatten() {
                            if dir_entry.path().is_file() {
                                if let Ok(meta) = dir_entry.metadata() {
                                    version_bytes += meta.len()
                                }
                            }
                        }
                    }
                    versions += 1;
                    version.bytes = version_bytes;
                    artifact_bytes += version.bytes;
                }
                artifacts += 1;
                artifact.bytes = artifact_bytes;
                group_bytes += artifact.bytes;
            }
            group.bytes = group_bytes;
            repo_bytes += group.bytes;
        }
        self.artifacts = artifacts;
        self.versions = versions;
        self.bytes = repo_bytes;
    }

    fn remove_versions(&self, dry_run: bool, list: bool) {
        let progress_bar = if !dry_run && !list && self.versions > PROGRESS_BAR_THRESHOLD {
            Some(
                ProgressBar::new(self.versions as u64)
                    .with_prefix("Remove artifacts")
                    .with_style(
                        ProgressStyle::default_bar()
                            .progress_chars("#>-")
                            .template("{prefix} [{wide_bar:.green/yellow}] {pos:>6}/{len:6}")
                            .unwrap(),
                    ),
            )
        } else {
            None
        };

        for group in self.groups.values() {
            for artifact in group.artifacts.values() {
                for version in artifact.versions.values() {
                    if list {
                        if let Some(str) = version.path.as_os_str().to_str() {
                            println!("{}", str);
                        }
                    } else if let Some(progress_bar) = &progress_bar {
                        progress_bar.inc(1);
                    }
                    if !dry_run {
                        if let Err(_error) = remove_dir_all(version.path.as_path()) {
                            if let Some(_path) = version.path.as_path().to_str() {
                                // TODO error handling / reporting
                            }
                        }
                    }
                }
            }
        }

        if let Some(progress_bar) = &progress_bar {
            progress_bar.finish_and_clear();
        }
    }

    fn prune_empty_directories(&self) {
        for group in self.groups.values() {
            for artifact in group.artifacts.values() {
                for version in artifact.versions.values() {
                    let _ = remove_dir(version.path.as_path());
                }
                // TODO What if only maven metadata is left?
                let _ = remove_dir(artifact.path.as_path());
            }
            let _ = remove_dir(group.path.as_path());
        }
    }
}

// ------------------------------------------------------ GAV

#[derive(Debug)]
pub struct Gav {
    pub group: Group,
    pub artifact: Artifact,
    pub version: Version,
}

impl Display for Gav {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.group.id, self.artifact.id, self.version)
    }
}
