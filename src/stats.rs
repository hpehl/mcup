use std::time::Duration;

use console::{pad_str, Alignment, Style};
use indicatif::{DecimalBytes, HumanDuration};

use crate::artifact::Artifact;
use crate::group::Group;
use crate::repo::Repository;
use crate::version::Version;

pub fn summary(repository: &Repository, duration: Duration, dry_run: bool) {
    let cyan = Style::new().cyan();
    let bytes = DecimalBytes(repository.bytes);
    let duration = HumanDuration(duration);
    println!(
        "The operation {}",
        if dry_run { "would affect " } else { "affects" }
    );
    println!();
    println!("    {} groups,", cyan.apply_to(repository.groups.len()));
    println!("    {} artifacts and", cyan.apply_to(repository.artifacts));
    println!("    {} versions", cyan.apply_to(repository.versions));
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

pub fn du(repository: &Repository, groups: bool, artifacts: bool, versions: bool) {
    let cyan_bold = Style::new().cyan().bold();
    let cyan = Style::new().cyan();
    let green = Style::new().green();
    let yellow = Style::new().yellow();

    print_bytes(
        repository.bytes,
        &repository.path.display().to_string().as_str(),
        &cyan_bold,
    );

    let mut sorted_groups: Vec<&Group> = repository.groups.values().collect();
    sorted_groups.sort_by(|a, b| b.bytes.cmp(&a.bytes));
    for group in sorted_groups {
        if group.bytes == 0 {
            continue;
        }
        if groups {
            print_bytes(group.bytes, group.id.as_str(), &cyan);
        }

        let mut sorted_artifacts: Vec<&Artifact> = group.artifacts.values().collect();
        sorted_artifacts.sort_by(|a, b| b.bytes.cmp(&a.bytes));
        for artifact in sorted_artifacts {
            if artifact.bytes == 0 {
                continue;
            }
            if artifacts {
                let name = if groups {
                    artifact.id.clone()
                } else {
                    format!("{}:{}", group.id.clone(), artifact.id.clone())
                };
                print_bytes(artifact.bytes, name.as_str(), &green);
            }

            let mut sorted_versions: Vec<&Version> = artifact.versions.values().collect();
            sorted_versions.reverse();
            for version in sorted_versions {
                if version.bytes == 0 {
                    continue;
                }
                if versions {
                    let name = if groups {
                        if artifacts {
                            version.to_string().clone()
                        } else {
                            format!("{}:{}", artifact.id.clone(), version.to_string().clone())
                        }
                    } else {
                        if artifacts {
                            version.to_string().clone()
                        } else {
                            format!(
                                "{}:{}:{}",
                                group.id.clone(),
                                artifact.id.clone(),
                                version.to_string()
                            )
                        }
                    };
                    print_bytes(version.bytes, name.as_str(), &yellow);
                }
            }
        }
    }
}

fn print_bytes(bytes: u64, name: &str, style: &Style) {
    let db = DecimalBytes(bytes);
    println!(
        "{} │ {}",
        pad_str(
            &format!("{}", style.apply_to(db)),
            10,
            Alignment::Right,
            None,
        ),
        name
    );
}
