use std::path::{Path, PathBuf, MAIN_SEPARATOR};

use clap::ArgMatches;
use walkdir::DirEntry;

use crate::artifact::Artifact;

pub struct GroupFilter {
    pub group_id: String,
    pub group_path: PathBuf,
}

impl GroupFilter {
    pub fn from(args: &ArgMatches, local_repo: &Path) -> Option<GroupFilter> {
        if let Some(group_id) = args.value_of("groups") {
            let group_path =
                local_repo.join(group_id.replace('.', MAIN_SEPARATOR.to_string().as_str()));
            Some(GroupFilter {
                group_id: group_id.to_string(),
                group_path,
            })
        } else {
            None
        }
    }

    pub fn subgroup_of(&self, dir_entry: &DirEntry) -> bool {
        self.group_path.starts_with(dir_entry.path())
            || dir_entry.path().starts_with(self.group_path.as_path())
    }

    pub fn no_subgroup_of(&self, dir_entry: &DirEntry) -> bool {
        !dir_entry.path().starts_with(self.group_path.as_path())
    }

    pub fn match_group_id(&self, artifact: &Artifact) -> bool {
        artifact.group_id.starts_with(self.group_id.as_str())
    }
}
