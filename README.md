[![Build status](https://github.com/BurntSushi/ripgrep/workflows/ci/badge.svg)](https://github.com/BurntSushi/ripgrep/actions)
[![Crates.io](https://img.shields.io/crates/v/mcup.svg)](https://crates.io/crates/mcup)
# Maven Clean Up

`mcup` /m'kʌp/ (**m**aven **c**lean **up**) is a command line tool to keep your local maven repository small 📦 and tidy 🧹.

[Maven](https://maven.apache.org/) is a great tool to build software for Java and other languages running on the JVM. At its core Maven is a dependency manager which downloads and stores dependencies in a local repository. Over time, this repository grows and takes up more and more space on the hard disk. It often contains obsolete versions or dependencies that are no longer needed.

`mcup` helps you to clean up your local repository. It uses filters to select artifacts based on the Maven coordinates `groupId`, `artifactId` and `version`. It knows three modes:

1. analyze and report the size of the artifacts selected by the filters
2. remove all artifacts selected by the filters and keep the rest
3. keep all artifacts selected by the filters and remove the rest

## Installation

Support for various package managers is _wip_.

[Precompiled binaries](https://github.com/hpehl/mcup/releases) are available for Windows, macOS and Linux. 

### Shell Completion

The release binaries are packaged with a man page and shell completion files for bash, fish, zsh, elvish and Powershell. To install them:

* for bash, move `completions/mcup.bash` to `$XDG_CONFIG_HOME/bash_completion` or `/etc/bash_completion.d/`.
* for fish, move `completions/mcup.fish` to `~/.config/fish/completions/`.
* for zsh, move `completions/_mcup` to one of your `$fpath` directories.
* for elvish, install `completions/mcup.elv` with [epm](https://elv.sh/ref/epm.html)
* for PowerShell, add `completions/_mcup.ps1` to your PowerShell profile.

### Cargo

If you're a Rust programmer, mcup can also be installed using `cargo`:

```
$ cargo install mcup
```

## Usage

```shell
mcup [FLAGS] [OPTIONS] <SUBCOMMAND>
```

### Flags

* `-r, --releases` Selects released artifacts only

* `-s, --snapshots` Selects snapshot artifacts only

* `-h, --help` Prints help information

* `-V, --version` Prints version information

### Options

* `-g, --groups <GROUPS>` Selects artifacts based on the group ID. 

  Subgroups are included by default.

  | Group            | Selection                                                    |
  | ---------------- | ------------------------------------------------------------ |
  | org              | All groups starting with 'org' (including 'org')             |
  | org.wildfly      | All groups starting with 'org.wildfly' (includes 'org.wildfly) |
  | org.wildfly.core | All groups starting with 'org.wildfly.core' (includes 'org.wildfly.core') |

* `-a, --artifacts <ARTIFACTS>`  Selects artifacts based on the artifact ID. 

  Supports globbing like in `maven-*-plugin` (see https://docs.rs/glob/0.3.0/glob/ for more details).

  | Artifact     | Selection                          |
  | ------------ | ---------------------------------- |
  | wildfly-core | Artifact 'wildfly-core' only       |
  | \*wildfly\*  | All artifacts containing 'wildfly' |

* `-v, --versions <VERSIONS>` Selects artifacts based on version (ranges). 

  Use `<n>..` to select the _n_ most recent versions, `..<n>` to select the _n_ oldest versions and `<version>` to select one specific _version_ only.

  | Version | Selection                  |
  | ------- | -------------------------- |
  | 1..     | The latest version         |
  | 5..     | The 5 most recent versions |
  | ..1     | The oldest version         |
  | ..4     | The 4 oldest versions      |
  | 1.2.3   | Version 1.2.3              |

* `-l, --local-repository <LOCAL_REPOSITORY>` Sets the location of the local maven repository.

  `mcup` respects the configuration of the local repository according to https://maven.apache/org/guides/mini/guide-configuring-maven.html#configuring-your-local-repository. The location of the local repository is computed in this order:

  1. The value of the option `--local-repository`
  1. The value of `<localRepository/>` in `~/.m2/settings.xml`
  1. Fall back to `~/.m2/repository/`

### Subcommands

#### Disk Usage (du)

Use this subcommand to analyze the disk usage of the artifacts selected by the filters. The subcommand accepts the following options:

* ` -o, --output <OUTPUT>` Defines whether (g)roups, (a)rtifacts and (v)ersions are included in the usage summary. Defaults to `ga`.

#### Keep / Remove (keep, rm)

Use one of these subcommands to remove artifacts selected by the filters:

* `keep` Keeps the artifacts matched by the filters and removes the rest
* `rm` Removes the artifacts matched by the filters and keeps the rest

The subcommands accept the following flags:

* `-d, --dry-run` Does not remove artifacts

* `--list` Prints the full path to the artifacts that will be removed.

  Use this flag together with `--dry-run` to review or post-process artifacts that will be removed:

  ```shell
  mcup --dry-run --list --versions '3..' keep > artifacts.txt
  ```

## Filter Combinations

At least one of `--releases`, `--snapshots`, `--groups`, `--artifacts` or `--versions` is required, where `--releases` and `--snapshots` are mutually exclusive.

If `--groups` is specified together with any other filter, only artifacts *below* the matched (sub)groups are subject to the subcommands (`keep` resp. `rm`). Artifacts *outside* the matched (sub)groups won't be touched. 

The following table explains the different filter combinations and describes which artifacts are kept resp. removed.

| Filter | keep | rm |
|---|---|---|
| `--groups` only | Keeps the specified (sub)groups and removes anything else. | Removes the specified (sub)groups. |
| `--artifacts` only | Keeps the specified artifacts and removes anything else. | Removes the specified artifacts. |
| `--versions` only | Keeps the specified versions and removes anything else. | Removes the specified versions. |
| `--groups` plus any other filter | Keeps the artifacts matched by the filters *below* the specified (sub)groups and removes anything else. | Removes the artifacts matched by the filters *below* the specified (sub)groups and keeps anything else. |
| All other combinations w/o `--groups` | Keeps the artifacts matched by the filters and removes anything else. | Removes the artifacts matched by the filters. |

## Examples

Keep the three most recent versions 

```shell
mcup --versions '3..' keep
```

Remove the three oldest versions

```shell
mcup --versions '..3' rm
```

Keep the latest releases (doesn't touch snapshots)

```shell
mcup --releases --version '1..' keep
```

Remove all snapshots

```shell
mcup --snapshots rm
```

Remove all artifacts starting with group ID 'edu'

```shell
mcup --groups edu rm
```

Keep the latest maven plugins. Don't remove anything outside group 'org.apache.maven.plugins'

```shell
mcup --groups 'org.apache.maven.plugins' --versions '1..' keep
```

Remove all artifacts (across all groups) starting with 'junit'

```shell
mcup --artifacts 'junit*' rm
```

