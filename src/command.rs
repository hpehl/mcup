use std::cmp::min;

use clap::ArgMatches;

use crate::command::Command::{Keep, Remove};
use crate::version::VersionRange;
use crate::version::VersionRange::{Latest, Oldest};

#[derive(PartialEq)]
pub enum Command {
    Keep,
    Remove,
}

impl Command {
    pub fn from(args: &ArgMatches) -> Command {
        if args.subcommand_matches("keep").is_some() {
            Keep
        } else if args.subcommand_matches("rm").is_some() {
            Remove
        } else {
            // Should not happen, since we use
            // AppSettings::SubcommandRequired
            panic!("No subcommand!")
        }
    }

    pub fn removals<'a, T>(&self, version_range: &VersionRange, slice: &'a [T]) -> &'a [T] {
        match (self, version_range) {
            (Keep, Latest(n)) => {
                if *n >= slice.len() {
                    &[]
                } else {
                    slice[*n..].as_ref()
                }
            }
            (Remove, Latest(n)) => {
                let n = min(*n, slice.len());
                slice[0..n].as_ref()
            }
            (Keep, Oldest(n)) => {
                if *n > slice.len() {
                    &[]
                } else {
                    let n = slice.len() - n;
                    slice[0..n].as_ref()
                }
            }
            (Remove, Oldest(n)) => {
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

        assert_eq!(vec![3, 2, 1], Keep.removals(&Latest(1), &versions));
        assert_eq!(vec![2, 1], Keep.removals(&Latest(2), &versions));
        assert_eq!(vec![1], Keep.removals(&Latest(3), &versions));
        assert!(Keep.removals(&Latest(4), &versions).is_empty());
        assert!(Keep.removals(&Latest(5), &versions).is_empty());
    }

    #[test]
    fn keep_oldest() {
        let versions = vec![4, 3, 2, 1];

        assert_eq!(vec![4, 3, 2], Keep.removals(&Oldest(1), &versions));
        assert_eq!(vec![4, 3], Keep.removals(&Oldest(2), &versions));
        assert_eq!(vec![4], Keep.removals(&Oldest(3), &versions));
        assert!(Keep.removals(&Oldest(4), &versions).is_empty());
        assert!(Keep.removals(&Oldest(5), &versions).is_empty());
    }

    #[test]
    fn remove_latest() {
        let versions = vec![4, 3, 2, 1];

        assert_eq!(vec![4], Remove.removals(&Latest(1), &versions));
        assert_eq!(vec![4, 3], Remove.removals(&Latest(2), &versions));
        assert_eq!(vec![4, 3, 2], Remove.removals(&Latest(3), &versions));
        assert_eq!(vec![4, 3, 2, 1], Remove.removals(&Latest(4), &versions));
        assert_eq!(vec![4, 3, 2, 1], Remove.removals(&Latest(5), &versions));
    }

    #[test]
    fn remove_oldest() {
        let versions = vec![4, 3, 2, 1];

        assert_eq!(vec![1], Remove.removals(&Oldest(1), &versions));
        assert_eq!(vec![2, 1], Remove.removals(&Oldest(2), &versions));
        assert_eq!(vec![3, 2, 1], Remove.removals(&Oldest(3), &versions));
        assert_eq!(vec![4, 3, 2, 1], Remove.removals(&Oldest(4), &versions));
        assert_eq!(vec![4, 3, 2, 1], Remove.removals(&Oldest(5), &versions));
    }
}
