use std::collections::HashMap;
use std::time::{Duration, Instant};

use console::Style;
use indicatif::{DecimalBytes, HumanDuration};

use crate::artifact::Artifact;
use crate::command::Command;

#[derive(Debug)]
pub struct Stats {
    pub command: Command,
    start: Instant,
    groups: HashMap<String, u64>,
    artifacts: HashMap<String, u64>,
    versions: HashMap<String, u64>,
    duration: Duration,
    errors: HashMap<String, String>,
}

impl Stats {
    pub fn start(command: Command) -> Stats {
        Stats {
            command,
            start: Instant::now(),
            groups: HashMap::new(),
            artifacts: HashMap::new(),
            versions: HashMap::new(),
            duration: Duration::ZERO,
            errors: HashMap::new(), // TODO Error reporting!
        }
    }

    pub fn add_artifact(&mut self, artifact: &Artifact) {
        let mut bytes: u64 = 0;
        if let Ok(read_dir) = artifact.version_path.read_dir() {
            for dir_entry in read_dir.flatten() {
                if dir_entry.path().is_file() {
                    if let Ok(meta) = dir_entry.metadata() {
                        bytes += meta.len()
                    }
                }
            }
        }
        *self
            .groups
            .entry(artifact.group_id.to_string())
            .or_insert(0) += bytes;
        *self
            .artifacts
            .entry(artifact.artifact_id.to_string())
            .or_insert(0) += bytes;
        *self
            .versions
            .entry(artifact.version.to_string())
            .or_insert(0) += bytes;
    }

    pub fn add_error(&mut self, path: String, error: String) {
        self.errors.insert(path, error);
    }

    pub fn finish(&mut self) {
        self.duration = self.start.elapsed();
    }

    pub fn summary(&self) {
        match self.command {
            Command::Keep(dry_run, _) | Command::Remove(dry_run, _) => {
                let cyan = Style::new().cyan();
                let bytes = DecimalBytes(self.versions.values().sum());
                let duration = HumanDuration(self.duration);
                println!(
                    "The operation {}",
                    if dry_run { "would affect " } else { "affects" }
                );
                println!();
                println!("    {} groups,", cyan.apply_to(self.groups.len()));
                println!("    {} artifacts and", cyan.apply_to(self.artifacts.len()));
                println!("    {} versions", cyan.apply_to(self.versions.len()));
                println!();
                println!(
                    "The operation took {} and {}released {}.",
                    cyan.apply_to(duration),
                    if dry_run { "would have " } else { "" },
                    cyan.apply_to(bytes)
                );
                if dry_run {
                    println!(
                        "Since you've used {} no artifacts have been removed.",
                        cyan.apply_to("--dry-run")
                    );
                }
            }
            Command::Du => {
                println!("DU");
                println!("{:#?}", self);
            }
        }
    }
}
