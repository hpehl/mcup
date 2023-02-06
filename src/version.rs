use fmt::Display;
use std::cmp::min;
use std::fmt;
use std::fmt::Formatter;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use clap::ArgMatches;

use crate::version::ReleaseType::{Releases, Snapshots};
use crate::version::VersionRange::{Exact, Latest, Oldest};
use std::hash::{Hash, Hasher};

// ------------------------------------------------------ version

// Maven version number (unfortunately we cannot use SemVer here)
#[derive(Debug, Clone, Ord, PartialOrd)]
pub struct Version {
    pub major: Option<u32>,
    pub minor: Option<u32>,
    pub patch: Option<u32>,
    pub qualifier: Option<String>,
    pub snapshot: bool,
    pub path: PathBuf,
    pub bytes: u64,
}

impl Version {
    pub fn from_path(path: &Path) -> Result<Version> {
        let version = path
            .file_name()
            .with_context(|| "No version")?
            .to_str()
            .with_context(|| "No version")?;
        let mut version = Version::from_str(version)?;
        version.path = path.to_path_buf();
        Ok(version)
    }

    //noinspection DuplicatedCode
    pub fn from_str(version: &str) -> Result<Version> {
        let input = version;
        let (version, snapshot) = match version.strip_suffix("-SNAPSHOT") {
            Some(ver) => (ver, true),
            None => (version, false),
        };

        let mut index = 0;
        let mut current = version;
        let mut mmp: [Option<u32>; 3] = [None; 3];
        let mut qual: Option<String> = None;

        loop {
            let dot = current.find('.');
            let dash = current.find('-');

            if let Some(dot_value) = dot {
                if let Some(dash_value) = dash {
                    // both '.' and '-'
                    if dot_value < dash_value {
                        // '.' before '-'
                        let (left, right) = current.split_once('.').unwrap();
                        match left.parse::<u32>() {
                            Ok(n) => mmp[index] = Some(n),
                            Err(_) => {
                                qual = Some(String::from(right));
                                break;
                            }
                        }
                        current = right;
                    } else {
                        // '-' before '.'
                        let (left, right) = current.split_once('-').unwrap();
                        match left.parse::<u32>() {
                            Ok(n) => {
                                mmp[index] = Some(n);
                                qual = Some(String::from(right));
                            }
                            Err(_) => qual = Some(String::from(current)),
                        }
                        break;
                    }
                } else {
                    // just '.'
                    let (left, right) = current.split_once('.').unwrap();
                    match left.parse::<u32>() {
                        Ok(n) => mmp[index] = Some(n),
                        Err(_) => {
                            qual = Some(String::from(right.trim_matches('.')));
                            break;
                        }
                    }
                    current = right;
                }
            } else if let Some(_dash_value) = dash {
                // just '-'
                let (left, right) = current.split_once('-').unwrap();
                match left.parse::<u32>() {
                    Ok(n) => {
                        mmp[index] = Some(n);
                        qual = Some(String::from(right));
                    }
                    Err(_) => qual = Some(String::from(current)),
                }
                break;
            } else {
                // neither '.' nor '-'
                match current.parse::<u32>() {
                    Ok(n) => {
                        mmp[index] = Some(n);
                        if version.ends_with(current) {
                            break;
                        }
                    }
                    Err(_) => {
                        qual = Some(String::from(current));
                        break;
                    }
                }
            }

            index += 1;
            if index > 2 {
                qual = Some(String::from(current));
                break;
            }
        }

        match mmp[0] {
            Some(_) => Ok(Version {
                major: mmp[0],
                minor: mmp[1],
                patch: mmp[2],
                qualifier: qual,
                snapshot,
                path: PathBuf::new(),
                bytes: 0,
            }),
            None => bail!("Invalid version: '{}'", input),
        }
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major
            && self.minor == other.minor
            && self.patch == other.patch
            && self.qualifier == other.qualifier
            && self.snapshot == other.snapshot
    }
}

impl Eq for Version {}

impl Hash for Version {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.major.hash(state);
        self.minor.hash(state);
        self.patch.hash(state);
        self.qualifier.hash(state);
        self.snapshot.hash(state);
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(i) = self.major {
            write!(f, "{i}")?;
        }
        if let Some(i) = self.minor {
            write!(f, ".{i}")?;
        }
        if let Some(i) = self.patch {
            write!(f, ".{i}")?;
        }
        if let Some(q) = &self.qualifier {
            write!(f, ".{q}")?;
        }
        write!(f, "{}", if self.snapshot { "-SNAPSHOT" } else { "" })
    }
}

// ------------------------------------------------------ version range

#[derive(Debug, Eq, PartialEq)]
pub enum VersionRange {
    Latest(usize),
    Oldest(usize),
    Exact(Version),
}

impl VersionRange {
    pub fn from(args: &ArgMatches) -> Option<VersionRange> {
        if let Some(version) = args.get_one::<String>("versions") {
            VersionRange::parse(version).ok()
        } else {
            None
        }
    }

    pub fn parse(version: &str) -> Result<VersionRange> {
        if let Some(count) = version.strip_suffix("..") {
            VersionRange::extract_versions(version, count, Latest)
        } else if let Some(count) = version.strip_prefix("..") {
            VersionRange::extract_versions(version, count, Oldest)
        } else {
            match Version::from_str(version) {
                Ok(v) => Ok(Exact(v)),
                Err(e) => bail!("{}", e),
            }
        }
    }

    fn extract_versions(
        version: &str,
        count: &str,
        range_fn: fn(usize) -> VersionRange,
    ) -> Result<VersionRange> {
        match count.parse::<usize>() {
            Ok(n) => {
                if n < 1 {
                    bail!("Illegal version range: {}. Version must be >= 1.", version)
                } else {
                    Ok(range_fn(n))
                    // Ok(Latest(n))
                }
            }
            Err(_) => bail!("Illegal version range: {}", version),
        }
    }

    // Select elements from the slice according to the version range
    pub fn select<'a, T>(&self, slice: &'a [T]) -> &'a [T] {
        match self {
            Latest(n) => {
                let n = min(*n, slice.len());
                slice[0..n].as_ref()
            }
            Oldest(n) => {
                if *n > slice.len() {
                    slice
                } else {
                    let n = slice.len() - *n;
                    slice[n..].as_ref()
                }
            }
            _ => slice,
        }
    }
}

// ------------------------------------------------------ release type

#[derive(Debug, Eq, PartialEq)]
pub enum ReleaseType {
    Releases,
    Snapshots,
}

impl ReleaseType {
    pub fn from(args: &ArgMatches) -> Option<ReleaseType> {
        if args.get_flag("releases") {
            Some(Releases)
        } else if args.get_flag("snapshots") {
            Some(Snapshots)
        } else {
            None
        }
    }
}

// ------------------------------------------------------ version tests

#[cfg(test)]
mod version_tests {
    use crate::version::Version;

    #[test]
    fn invalid_version() {
        assert!(Version::from_str("").is_err());
        assert!(Version::from_str(".").is_err());
        assert!(Version::from_str("..").is_err());
        assert!(Version::from_str("-").is_err());
        assert!(Version::from_str("--").is_err());
        assert!(Version::from_str(".-").is_err());
        assert!(Version::from_str("-.").is_err());
        assert!(Version::from_str("1a").is_err());
        assert!(Version::from_str("a").is_err());
        assert!(Version::from_str("a.b").is_err());
        assert!(Version::from_str("a-c").is_err());
        assert!(Version::from_str("-SNAPSHOT").is_err());
        assert!(Version::from_str("--SNAPSHOT").is_err());
        assert!(Version::from_str("1SNAPSHOT").is_err());
        assert!(Version::from_str("xSNAPSHOT").is_err());
        assert!(Version::from_str("x-SNAPSHOT").is_err());
    }

    #[test]
    fn major_minor_patch() {
        assert_version("1", Some(1), None, None, None, false);
        assert_version("1.2", Some(1), Some(2), None, None, false);
        assert_version("1.2.3", Some(1), Some(2), Some(3), None, false);
    }

    #[test]
    fn version_dots() {
        assert_version("1.Final", Some(1), None, None, Some("Final"), false);
        assert_version("1.4alpha", Some(1), None, None, Some("4alpha"), false);
        assert_version("1.4.alpha", Some(1), Some(4), None, Some("alpha"), false);
        assert_version("1.2.3.4", Some(1), Some(2), Some(3), Some("4"), false);
        assert_version(
            "1.2.3.Final",
            Some(1),
            Some(2),
            Some(3),
            Some("Final"),
            false,
        );
        assert_version("1..2", Some(1), None, None, Some("2"), false);
        assert_version("1...2", Some(1), None, None, Some("2"), false);
    }

    #[test]
    fn version_dashes() {
        assert_version("1-Final", Some(1), None, None, Some("Final"), false);
        assert_version("1-4alpha", Some(1), None, None, Some("4alpha"), false);
        assert_version("1-4-alpha", Some(1), None, None, Some("4-alpha"), false);
        assert_version("1--2", Some(1), None, None, Some("-2"), false);
        assert_version("1---2", Some(1), None, None, Some("--2"), false);
    }

    #[test]
    fn version_mixed() {
        assert_version("1.2-a", Some(1), Some(2), None, Some("a"), false);
        assert_version("1-2.a", Some(1), None, None, Some("2.a"), false);
        assert_version("1.2-a.1", Some(1), Some(2), None, Some("a.1"), false);
        assert_version("1-2.a-1", Some(1), None, None, Some("2.a-1"), false);
    }

    #[test]
    fn version_order() {
        let mut versions = vec![
            Version::from_str("1").unwrap(),
            Version::from_str("2").unwrap(),
            Version::from_str("1.0").unwrap(),
            Version::from_str("1.1").unwrap(),
            Version::from_str("1.2").unwrap(),
            Version::from_str("1.3").unwrap(),
            Version::from_str("1.0.0").unwrap(),
            Version::from_str("1.0.1").unwrap(),
            Version::from_str("1.0.2").unwrap(),
            Version::from_str("1.0.3").unwrap(),
            Version::from_str("1.0.0.Alpha").unwrap(),
            Version::from_str("1.0.0-Beta").unwrap(),
            Version::from_str("1.0.0.Final").unwrap(),
            Version::from_str("1.0.0-SNAPSHOT").unwrap(),
        ];

        versions.sort();
        assert_eq!(
            vec![
                Version::from_str("1").unwrap(),
                Version::from_str("1.0").unwrap(),
                Version::from_str("1.0.0").unwrap(),
                Version::from_str("1.0.0-SNAPSHOT").unwrap(),
                Version::from_str("1.0.0.Alpha").unwrap(),
                Version::from_str("1.0.0-Beta").unwrap(),
                Version::from_str("1.0.0.Final").unwrap(),
                Version::from_str("1.0.1").unwrap(),
                Version::from_str("1.0.2").unwrap(),
                Version::from_str("1.0.3").unwrap(),
                Version::from_str("1.1").unwrap(),
                Version::from_str("1.2").unwrap(),
                Version::from_str("1.3").unwrap(),
                Version::from_str("2").unwrap(),
            ],
            versions
        );
    }

    fn assert_version(
        version: &str,
        major: Option<u32>,
        minor: Option<u32>,
        incremental: Option<u32>,
        qualifier: Option<&str>,
        snapshot: bool,
    ) {
        let v = Version::from_str(version).expect("Invalid version");
        assert_eq!(major, v.major);
        assert_eq!(minor, v.minor);
        assert_eq!(incremental, v.patch);
        assert_eq!(qualifier.map(|s| String::from(s)), v.qualifier);
        assert_eq!(snapshot, v.snapshot);
    }
}

// ------------------------------------------------------ version range tests

#[cfg(test)]
mod version_range_tests {
    use crate::version::VersionRange::{Exact, Latest, Oldest};
    use crate::version::{Version, VersionRange};

    #[test]
    fn invalid_version_range() {
        assert!(VersionRange::parse("").is_err());
        assert!(VersionRange::parse("foo").is_err());
        assert!(VersionRange::parse("a..").is_err());
        assert!(VersionRange::parse("..a").is_err());
        assert!(VersionRange::parse("-1..").is_err());
        assert!(VersionRange::parse("0..").is_err());
        assert!(VersionRange::parse("..-1").is_err());
        assert!(VersionRange::parse("..0").is_err());
        assert!(VersionRange::parse("1...").is_err());
        assert!(VersionRange::parse("...1").is_err());
        assert!(VersionRange::parse("1.1..").is_err());
        assert!(VersionRange::parse("..1.1").is_err());
    }

    #[test]
    fn parse_latest() {
        assert_eq!(Latest(1), VersionRange::parse("1..").unwrap());
        assert_eq!(Latest(2), VersionRange::parse("2..").unwrap());
        assert_eq!(Latest(3), VersionRange::parse("3..").unwrap());
    }

    #[test]
    fn parse_oldest() {
        assert_eq!(Oldest(1), VersionRange::parse("..1").unwrap());
        assert_eq!(Oldest(2), VersionRange::parse("..2").unwrap());
        assert_eq!(Oldest(3), VersionRange::parse("..3").unwrap());
    }

    #[test]
    fn parse_exact() {
        let version = Version::from_str("1.2.3").unwrap();
        assert_eq!(Exact(version), VersionRange::parse("1.2.3").unwrap());
    }

    #[test]
    fn select_latest() {
        let versions = vec![4, 3, 2, 1];

        assert_eq!(vec![4], Latest(1).select(&versions));
        assert_eq!(vec![4, 3], Latest(2).select(&versions));
        assert_eq!(vec![4, 3, 2], Latest(3).select(&versions));
        assert_eq!(vec![4, 3, 2, 1], Latest(4).select(&versions));
        assert_eq!(vec![4, 3, 2, 1], Latest(5).select(&versions));
    }

    #[test]
    fn select_oldest() {
        let versions = vec![4, 3, 2, 1];

        assert_eq!(vec![1], Oldest(1).select(&versions));
        assert_eq!(vec![2, 1], Oldest(2).select(&versions));
        assert_eq!(vec![3, 2, 1], Oldest(3).select(&versions));
        assert_eq!(vec![4, 3, 2, 1], Oldest(4).select(&versions));
        assert_eq!(vec![4, 3, 2, 1], Oldest(5).select(&versions));
    }
}
