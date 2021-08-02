# Maven Clean Up

`mcup` /m'kʌp/ (**m**aven **c**lean **up**) is a command line tool to keep your local maven repository small and tidy.

[Maven](https://maven.apache.org/) is a great tool to build software for Java and other languages running on the JVM. At its core Maven is a dependency manager which downloads and stores dependencies in a local repository. Over time, this repository grows and takes up more and more space on the hard disk. It often contains obsolete versions or dependencies that are no longer needed.

`mcup` helps you to clean up your local repository. It uses filters to select artifacts based on the Maven 
coordinates `groupId`, `artifactId` and `version`. It knows two modes:

1. remove all artifacts selected by the filters and keep the rest
2. keep all artifacts selected by the filters and remove the rest

## Installation

Pending...

### Shell Completions

Shell completion files are available for Bash, Fish, Zsh and PowerShell.

For `bash`, move `mcup.bash` to `$XDG_CONFIG_HOME/bash_completion`
or `/etc/bash_completion.d/`.

For `zsh`, move `_mcup` to one of your `$fpath` directories.

For `fish`, move `mcup.fish` to `$HOME/.config/fish/completions`.

For `elvish`, install `mcup.elv` using the elvish package manager.

## Usage

```shell
mcup [FLAGS] [OPTIONS] <SUBCOMMAND>
```

### Flags

`-r, --releases`  
Selects released artifacts only

`-s, --snapshots`  
Selects snapshot artifacts only

`-d, --dry-run`  
Does not remove artifacts

`--list`  
Prints the full path to the artifacts that will be removed. Normally `mcup` shows a progress bar while removing the selected artifacts. This flag disables the progress bar and prints the full path to the artifacts that will be removed.

Use this flag together with `--dry-run` to review or post-process artifacts that will be removed:

```shell
mcup --dry-run --list --versions '3..' keep > artifacts.txt
```

`-h, --help`  
Prints help information

`-V, --version`  
Prints version information

### Options

`-g, --groups <GROUPS>`  
Selects artifacts based on the group ID. Subgroups are included by default. Subgroups are included by default.

`-a, --artifacts <ARTIFACTS>`  
Selects artifacts based on the artifact ID. Supports globbing like in `maven-*-plugin`.

`-v, --versions <VERSIONS>`  
Selects artifacts based on version (ranges). Use `<n>..` to select the _n_ most recent versions, `..<n>` to select the _n_ oldest versions and `<version>` to select one specific _version_ only.

`-l, --local-repository <LOCAL_REPOSITORY>`  
Sets the location of the local maven repository. Respects the directory configured in `~/.m2/settings.xml`. Falls back to `~/.m2/repository`, if nothing has been specified or configured.

### Groups

Subgroups are included by default:

| Group | Selection |
|---|---|
| org | All groups starting with 'org' (including 'org') |
| org.wildfly | All groups starting with 'org.wildfly' (includes 'org.wildfly) |
| org.wildfly.core | All groups starting with 'org.wildfly.core' (includes 'org.wildfly.core') |

### Artifacts

Artifacts can be selected using globbing:

| Artifact | Selection |
|---|---|
| wildfly-core | Artifact 'wildfly-core' only |
| \*wildfly\* | All artifacts containing 'wildfly' |

### Versions

Versions can be selected using latest, oldest or exact version: 

| Version | Selection |
|---|---|
| 1.. | The latest version |
| 5.. | The 5 most recent versions |
| ..1 | The oldest version |
| ..4 | The 4 oldest versions |
| 1.2.3 | Version 1.2.3 |

### Subcommands

Subcommand must be one of

- `keep`:  Keeps the artifacts matched by the filters and removes the rest
- `rm`: Removes the artifacts matched by the filters and keeps the rest

## Filter Combinations

At least one of `--releases`, `--snapshots`, `--groups`, `--artifacts` or `--versions` is required, where `--releases` and `--snapshots` are mutually exclusive.

If `--groups` is specified together with any other filter, only artifacts *below* the matched (sub)groups are 
subject to the subcommands (`keep` resp. `rm`). Artifacts *outside* the matched (sub)groups won't be touched. 

The following table explains the different filter combinations and describes which artifacts are kept resp. removed.

| Filter | keep | rm |
|---|---|---|
| `--groups` only | Keeps the specified (sub)groups and removes anything else. | Removes the specified (sub)groups. |
| `--artifacts` only | Keeps the specified artifacts and removes anything else. | Removes the specified artifacts. |
| `--versions` only | Keeps the specified versions and removes anything else. | Removes the specified versions. |
| `--groups` plus any other filter | Keeps the artifacts matched by the filters *below* the specified (sub)groups and removes anything else. | Removes the artifacts matched by the filters *below* the specified (sub)groups and keeps anything else. |
| All other combinations w/o `--groups` | Keeps the artifacts matched by the filters and removes anything else. | Removes the artifacts matched by the filters. |

## Local Repository

`mcup` respects the configuration of the local repository according to https://maven.apache/org/guides/mini/guide-configuring-maven.html#configuring-your-local-repository. You can overwrite the location of the local repository using the option `--local-repository`. 

Thus, the location of the local repository is computed in this order:

1. The value of the option `--local-repository`
2. The value of `<localRepository/>` in `~/.m2/settings.xml`
3. Fall back to `~/.m2/repository/`

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

