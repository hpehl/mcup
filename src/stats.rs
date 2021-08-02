use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

use console::Style;
use indicatif::{DecimalBytes, HumanDuration};

use crate::artifact::Artifact;

#[derive(Debug)]
pub struct Stats {
    start: Instant,
    groups: usize,
    artifacts: usize,
    versions: usize,
    bytes: u64,
    duration: Duration,
    errors: HashMap<String, String>,
    dry_run: bool,
}

impl Stats {
    pub fn start(dry_run: bool) -> Stats {
        Stats {
            start: Instant::now(),
            groups: 0,
            artifacts: 0,
            versions: 0,
            bytes: 0,
            duration: Duration::ZERO,
            errors: HashMap::new(), // TODO Error reporting!
            dry_run,
        }
    }

    pub fn update(&mut self, artifacts: &[Artifact]) {
        let mut groups = HashSet::new();
        let mut versions = HashSet::new();
        for artifact in artifacts {
            groups.insert(&artifact.group_id);
            versions.insert(&artifact.version);
        }

        self.groups = groups.len();
        self.artifacts = artifacts.len();
        self.versions = versions.len();
    }

    pub fn bytes(&mut self, artifact: &Artifact) {
        if let Ok(read_dir) = artifact.version_path.read_dir() {
            for dir_entry in read_dir.flatten() {
                if dir_entry.path().is_file() {
                    if let Ok(meta) = dir_entry.metadata() {
                        self.bytes += meta.len()
                    }
                }
            }
        }
    }

    pub fn error(&mut self, path: String, error: String) {
        self.errors.insert(path, error);
    }

    pub fn finish(&mut self) {
        self.duration = self.start.elapsed();
    }

    pub fn summary(&self) {
        let cyan = Style::new().cyan();
        let bytes = DecimalBytes(self.bytes);
        let duration = HumanDuration(self.duration);
        println!(
            "The operation {}",
            if self.dry_run {
                "would affect "
            } else {
                "affects"
            }
        );
        println!();
        println!("    {} groups,", cyan.apply_to(self.groups));
        println!("    {} artifacts and", cyan.apply_to(self.artifacts));
        println!("    {} versions", cyan.apply_to(self.versions));
        println!();
        println!(
            "The operation took {} and {}released {}.",
            cyan.apply_to(duration),
            if self.dry_run { "would have " } else { "" },
            cyan.apply_to(bytes)
        );
        if self.dry_run {
            println!(
                "Since you've used {} no artifacts have been removed.",
                cyan.apply_to("--dry-run")
            );
        }
    }
}
