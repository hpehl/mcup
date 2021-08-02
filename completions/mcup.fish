complete -c mcup -n "__fish_use_subcommand" -s g -l groups -d 'Selects artifacts based on the group ID. Subgroups are included by default.' -r
complete -c mcup -n "__fish_use_subcommand" -s a -l artifacts -d 'Selects artifacts based on the artifact ID. Supports globbing like in \'maven-*-plugin\'.' -r
complete -c mcup -n "__fish_use_subcommand" -s v -l versions -d 'Selects artifacts based on version (ranges). Use \'<n>..\' to select the n most recent versions, \'..<n>\' to select the n oldest versions and \'<version>\' to select one specific version only.' -r
complete -c mcup -n "__fish_use_subcommand" -s l -l local-repository -d 'Sets the location of the local maven repository. Respects the directory configured in \'~/.m2/settings.xml\'. Falls back to \'~/.m2/repository\', if nothing has been specified or configured.' -r
complete -c mcup -n "__fish_use_subcommand" -s r -l releases -d 'Selects released artifacts only'
complete -c mcup -n "__fish_use_subcommand" -s s -l snapshots -d 'Selects snapshot artifacts only'
complete -c mcup -n "__fish_use_subcommand" -s d -l dry-run -d 'Does not remove artifacts'
complete -c mcup -n "__fish_use_subcommand" -l list -d 'Prints the full path to the artifacts that will be removed'
complete -c mcup -n "__fish_use_subcommand" -s h -l help -d 'Prints help information'
complete -c mcup -n "__fish_use_subcommand" -s V -l version -d 'Prints version information'
complete -c mcup -n "__fish_use_subcommand" -f -a "keep" -d 'Keeps the artifacts matched by the filters and removes the rest'
complete -c mcup -n "__fish_use_subcommand" -f -a "rm" -d 'Removes the artifacts matched by the filters and keeps the rest'
complete -c mcup -n "__fish_seen_subcommand_from keep" -s h -l help -d 'Prints help information'
complete -c mcup -n "__fish_seen_subcommand_from rm" -s h -l help -d 'Prints help information'
