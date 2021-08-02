use fmt::Display;
use std::fmt;
use std::fmt::Formatter;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::ArgMatches;
use glob::Pattern;

use crate::version::Version;

#[derive(Debug, Clone)]
pub struct Artifact {
    pub group_path: PathBuf,
    pub group_id: String,
    pub artifact_path: PathBuf,
    pub artifact_id: String,
    pub version_path: PathBuf,
    pub version: Version,
}

impl Artifact {
    pub fn from<'a>(local_repo: &'a Path, pom: &'a Path) -> Result<Artifact> {
        let version_path = pom.parent().with_context(|| "No version path")?;
        let version = Version::parse(
            version_path
                .file_name()
                .with_context(|| "No version")?
                .to_str()
                .with_context(|| "No version")?,
        )?;

        let artifact_path = version_path.parent().with_context(|| "No artifact path")?;
        let artifact_id = artifact_path
            .file_name()
            .with_context(|| "No artifact ID")?
            .to_str()
            .with_context(|| "No artifact ID")?;

        let group_path = artifact_path.parent().with_context(|| "No group path")?;
        let group_id = group_path
            .strip_prefix(local_repo)?
            .components()
            .map(|c| c.as_os_str().to_str().unwrap_or(""))
            .collect::<Vec<&'a str>>()
            .join(".");

        Ok(Artifact {
            group_path: group_path.to_path_buf(),
            group_id,
            artifact_path: artifact_path.to_path_buf(),
            artifact_id: artifact_id.to_string(),
            version_path: version_path.to_path_buf(),
            version,
        })
    }
}

impl Display for Artifact {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.group_id, self.artifact_id, self.version)
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

    pub fn match_artifact_id(&self, artifact: &Artifact) -> bool {
        if let Ok(pattern) = Pattern::new(self.artifacts.as_str()) {
            pattern.matches(artifact.artifact_id.as_str())
        } else {
            false
        }
    }
}
