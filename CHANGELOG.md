# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

This release doesn't add new features, but upgrades dependencies only:

- anyhow: 1.0.42 → 1.0.44
- clap: 3.0.0-beta.2 → 3.0.0-beta.5
- clap_generate: 3.0.0-beta.2 → 3.0.0-beta.5
- console: 0.14.1 → 0.15.0
- directories: 3.0.2 → 4.0.1

## [0.2.0] - 2021-08-10

### Added
Add `du` subcommand to analyze the disk usage of the selected groups, artifacts and versions.

### Changed
Move `--dry-run` and `--list` flags to `keep` and `rm` subcommands.

## [0.1.0] - 2021-08-03
First public release

<!-- next-url -->
[Unreleased]: https://github.com/hpehl/mcup/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/hpehl/mcup/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/hpehl/mcup/releases/tag/v0.1.0
