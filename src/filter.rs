use std::path::Path;

use bit_vec::BitVec;
use clap::ArgMatches;

use Command::{Keep, Remove};

use crate::artifact::{Artifact, ArtifactFilter};
use crate::command::Command;
use crate::group::GroupFilter;
use crate::version::ReleaseType::{Releases, Snapshots};
use crate::version::VersionRange::Exact;
use crate::version::{ReleaseType, VersionRange};

pub struct Filter {
    pub group_filter: Option<GroupFilter>,
    pub artifact_filter: Option<ArtifactFilter>,
    pub version_range: Option<VersionRange>,
    pub release_type: Option<ReleaseType>,
}

impl Filter {
    pub fn from(local_repo: &Path, args: &ArgMatches) -> Filter {
        Filter {
            group_filter: GroupFilter::from(args, local_repo),
            artifact_filter: ArtifactFilter::from(args),
            version_range: VersionRange::from(args),
            release_type: ReleaseType::from(args),
        }
    }

    pub fn conjunction(&self, artifact: &Artifact, command: &Command) -> bool {
        let mut conditions = BitVec::new();
        if let Some(artifact_filter) = &self.artifact_filter {
            conditions.push(match command {
                Keep => !artifact_filter.match_artifact_id(artifact),
                Remove => artifact_filter.match_artifact_id(artifact),
            });
        }
        // if let Exact(version) = version_range {
        if let Some(Exact(version)) = &self.version_range {
            conditions.push(match command {
                Keep => *version != artifact.version,
                Remove => *version == artifact.version,
            });
        }
        if let Some(release_type) = &self.release_type {
            match release_type {
                Releases => conditions.push(match command {
                    Keep => artifact.version.snapshot,
                    Remove => !artifact.version.snapshot,
                }),
                Snapshots => conditions.push(match command {
                    Keep => !artifact.version.snapshot,
                    Remove => artifact.version.snapshot,
                }),
            }
        }
        conditions.all()
    }
}
