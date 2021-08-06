use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

use clap::ArgMatches;
use glob::Pattern;

use crate::version::Version;

#[derive(Debug)]
pub struct Artifact {
    pub id: String,
    pub path: PathBuf,
    pub versions: BTreeMap<Version, Version>,
    pub bytes: u64,
}

impl Artifact {
    pub fn new(id: &str, path: &Path) -> Artifact {
        Artifact {
            id: id.to_string(),
            path: path.to_path_buf(),
            versions: BTreeMap::new(),
            bytes: 0,
        }
    }
}

impl PartialEq for Artifact {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Artifact {}

impl Display for Artifact {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

pub struct ArtifactFilter {
    pub artifacts: String,
}

impl ArtifactFilter {
    pub fn from(args: &ArgMatches) -> Option<ArtifactFilter> {
        args.value_of("artifacts").map(|artifact| ArtifactFilter {
            artifacts: artifact.to_string(),
        })
    }

    pub fn match_artifact_id(&self, artifact_id: &str) -> bool {
        if let Ok(pattern) = Pattern::new(self.artifacts.as_str()) {
            pattern.matches(artifact_id)
        } else {
            false
        }
    }
}
