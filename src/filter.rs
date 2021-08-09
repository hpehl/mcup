use std::path::Path;

use bit_vec::BitVec;
use clap::ArgMatches;

use Command::{Du, Keep, Remove};

use crate::artifact::ArtifactFilter;
use crate::command::Command;
use crate::group::GroupFilter;
use crate::repo::Gav;
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
    pub fn from(args: &ArgMatches, local_repo: &Path) -> Filter {
        Filter {
            group_filter: GroupFilter::from(args, local_repo),
            artifact_filter: ArtifactFilter::from(args),
            version_range: VersionRange::from(args),
            release_type: ReleaseType::from(args),
        }
    }

    // combine given filters with &&
    pub fn conjunction(&self, gav: &Gav, command: &Command) -> bool {
        let mut conditions = BitVec::new();
        if let Some(artifact_filter) = &self.artifact_filter {
            conditions.push(match command {
                Keep(_, _) => !artifact_filter.match_artifact_id(gav.artifact.id.as_str()),
                Remove(_, _) | Du(_, _, _) => {
                    artifact_filter.match_artifact_id(gav.artifact.id.as_str())
                }
            });
        }
        // if let Exact(version) = version_range {
        if let Some(Exact(version)) = &self.version_range {
            conditions.push(match command {
                Keep(_, _) => *version != gav.version,
                Remove(_, _) | Du(_, _, _) => *version == gav.version,
            });
        }
        if let Some(release_type) = &self.release_type {
            match release_type {
                Releases => conditions.push(match command {
                    Keep(_, _) => gav.version.snapshot,
                    Remove(_, _) | Du(_, _, _) => !gav.version.snapshot,
                }),
                Snapshots => conditions.push(match command {
                    Keep(_, _) => !gav.version.snapshot,
                    Remove(_, _) | Du(_, _, _) => gav.version.snapshot,
                }),
            }
        }
        if conditions.is_empty() {
            true
        } else {
            conditions.all()
        }
    }
}
