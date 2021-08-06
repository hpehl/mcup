use std::cmp::min;

use clap::ArgMatches;

use crate::command::Command::{Du, Keep, Remove};
use crate::version::VersionRange;
use crate::version::VersionRange::{Latest, Oldest};

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Keep(bool, bool),
    Remove(bool, bool),
    Du,
}

impl Command {
    pub fn from(args: &ArgMatches) -> Command {
        if args.subcommand_matches("keep").is_some() {
            let sub_args = args.subcommand_matches("keep").unwrap();
            Keep(sub_args.is_present("dry-run"), sub_args.is_present("list"))
        } else if args.subcommand_matches("rm").is_some() {
            let sub_args = args.subcommand_matches("rm").unwrap();
            Remove(sub_args.is_present("dry-run"), sub_args.is_present("list"))
        } else if args.subcommand_matches("du").is_some() {
            Du
        } else {
            // Should not happen, since we use
            // AppSettings::SubcommandRequired
            panic!("No subcommand!")
        }
    }

    // Select elements from the slice according to the command and version range
    pub fn select<'a, T>(&self, version_range: &VersionRange, slice: &'a [T]) -> &'a [T] {
        match (self, version_range) {
            (Keep(_, _), Latest(n)) => {
                if *n >= slice.len() {
                    &[]
                } else {
                    slice[*n..].as_ref()
                }
            }
            (Remove(_, _), Latest(n)) => {
                let n = min(*n, slice.len());
                slice[0..n].as_ref()
            }
            (Keep(_, _), Oldest(n)) => {
                if *n > slice.len() {
                    &[]
                } else {
                    let n = slice.len() - n;
                    slice[0..n].as_ref()
                }
            }
            (Remove(_, _), Oldest(n)) => {
                if *n > slice.len() {
                    slice
                } else {
                    let n = slice.len() - *n;
                    slice[n..].as_ref()
                }
            }
            (_, _) => slice,
        }
    }
}

#[cfg(test)]
mod command_tests {
    use crate::command::Command::{Keep, Remove};
    use crate::version::VersionRange::{Latest, Oldest};

    #[test]
    fn keep_latest() {
        let versions = vec![4, 3, 2, 1];

        assert_eq!(vec![3, 2, 1], Keep.select(&Latest(1), &versions));
        assert_eq!(vec![2, 1], Keep.select(&Latest(2), &versions));
        assert_eq!(vec![1], Keep.select(&Latest(3), &versions));
        assert!(Keep.select(&Latest(4), &versions).is_empty());
        assert!(Keep.select(&Latest(5), &versions).is_empty());
    }

    #[test]
    fn keep_oldest() {
        let versions = vec![4, 3, 2, 1];

        assert_eq!(vec![4, 3, 2], Keep.select(&Oldest(1), &versions));
        assert_eq!(vec![4, 3], Keep.select(&Oldest(2), &versions));
        assert_eq!(vec![4], Keep.select(&Oldest(3), &versions));
        assert!(Keep.select(&Oldest(4), &versions).is_empty());
        assert!(Keep.select(&Oldest(5), &versions).is_empty());
    }

    #[test]
    fn remove_latest() {
        let versions = vec![4, 3, 2, 1];

        assert_eq!(vec![4], Remove.select(&Latest(1), &versions));
        assert_eq!(vec![4, 3], Remove.select(&Latest(2), &versions));
        assert_eq!(vec![4, 3, 2], Remove.select(&Latest(3), &versions));
        assert_eq!(vec![4, 3, 2, 1], Remove.select(&Latest(4), &versions));
        assert_eq!(vec![4, 3, 2, 1], Remove.select(&Latest(5), &versions));
    }

    #[test]
    fn remove_oldest() {
        let versions = vec![4, 3, 2, 1];

        assert_eq!(vec![1], Remove.select(&Oldest(1), &versions));
        assert_eq!(vec![2, 1], Remove.select(&Oldest(2), &versions));
        assert_eq!(vec![3, 2, 1], Remove.select(&Oldest(3), &versions));
        assert_eq!(vec![4, 3, 2, 1], Remove.select(&Oldest(4), &versions));
        assert_eq!(vec![4, 3, 2, 1], Remove.select(&Oldest(5), &versions));
    }
}
