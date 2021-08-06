complete -c mcup -n "__fish_use_subcommand" -s g -l groups -d 'Selects artifacts based on the group ID. Subgroups are included by default.' -r
complete -c mcup -n "__fish_use_subcommand" -s a -l artifacts -d 'Selects artifacts based on the artifact ID. Supports globbing like in \'maven-*-plugin\'.' -r
complete -c mcup -n "__fish_use_subcommand" -s v -l versions -d 'Selects artifacts based on version (ranges). Use \'<n>..\' to select the n most recent versions, \'..<n>\' to select the n oldest versions and \'<version>\' to select one specific version only.' -r
complete -c mcup -n "__fish_use_subcommand" -s l -l local-repository -d 'Sets the location of the local maven repository. Respects the directory configured in \'~/.m2/settings.xml\'. Falls back to \'~/.m2/repository\', if nothing has been specified or configured.' -r
complete -c mcup -n "__fish_use_subcommand" -s r -l releases -d 'Selects released artifacts only'
complete -c mcup -n "__fish_use_subcommand" -s s -l snapshots -d 'Selects snapshot artifacts only'
complete -c mcup -n "__fish_use_subcommand" -s h -l help -d 'Prints help information'
complete -c mcup -n "__fish_use_subcommand" -s V -l version -d 'Prints version information'
complete -c mcup -n "__fish_use_subcommand" -f -a "keep" -d 'Keeps the artifacts matched by the filters and removes the rest'
complete -c mcup -n "__fish_use_subcommand" -f -a "rm" -d 'Removes the artifacts matched by the filters and keeps the rest'
complete -c mcup -n "__fish_use_subcommand" -f -a "du" -d 'Analyzes the size of the artifacts selected by the filters'
complete -c mcup -n "__fish_use_subcommand" -f -a "help" -d 'Prints this message or the help of the given subcommand(s)'
complete -c mcup -n "__fish_seen_subcommand_from keep" -s d -l dry-run -d 'Does not remove artifacts'
complete -c mcup -n "__fish_seen_subcommand_from keep" -l list -d 'Prints the full path to the artifacts that will be removed'
complete -c mcup -n "__fish_seen_subcommand_from keep" -s h -l help -d 'Prints help information'
complete -c mcup -n "__fish_seen_subcommand_from rm" -s d -l dry-run -d 'Does not remove artifacts'
complete -c mcup -n "__fish_seen_subcommand_from rm" -l list -d 'Prints the full path to the artifacts that will be removed'
complete -c mcup -n "__fish_seen_subcommand_from rm" -s h -l help -d 'Prints help information'
complete -c mcup -n "__fish_seen_subcommand_from du" -s o -l output -d 'Defines whether (g)roups, (a)rtifacts and (v)ersions are included in the usage summary' -r
complete -c mcup -n "__fish_seen_subcommand_from du" -s h -l help -d 'Prints help information'
complete -c mcup -n "__fish_seen_subcommand_from help" -s h -l help -d 'Prints help information'
complete -c mcup -n "__fish_seen_subcommand_from help" -s V -l version -d 'Prints version information'
