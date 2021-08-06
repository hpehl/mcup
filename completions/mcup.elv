
edit:completion:arg-completer[mcup] = [@words]{
    fn spaces [n]{
        repeat $n ' ' | joins ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display-suffix=' '(spaces (- 14 (wcswidth $text)))$desc
    }
    command = 'mcup'
    for word $words[1:-1] {
        if (has-prefix $word '-') {
            break
        }
        command = $command';'$word
    }
    completions = [
        &'mcup'= {
            cand -g 'Selects artifacts based on the group ID. Subgroups are included by default.'
            cand --groups 'Selects artifacts based on the group ID. Subgroups are included by default.'
            cand -a 'Selects artifacts based on the artifact ID. Supports globbing like in ''maven-*-plugin''.'
            cand --artifacts 'Selects artifacts based on the artifact ID. Supports globbing like in ''maven-*-plugin''.'
            cand -v 'Selects artifacts based on version (ranges). Use ''<n>..'' to select the n most recent versions, ''..<n>'' to select the n oldest versions and ''<version>'' to select one specific version only.'
            cand --versions 'Selects artifacts based on version (ranges). Use ''<n>..'' to select the n most recent versions, ''..<n>'' to select the n oldest versions and ''<version>'' to select one specific version only.'
            cand -l 'Sets the location of the local maven repository. Respects the directory configured in ''~/.m2/settings.xml''. Falls back to ''~/.m2/repository'', if nothing has been specified or configured.'
            cand --local-repository 'Sets the location of the local maven repository. Respects the directory configured in ''~/.m2/settings.xml''. Falls back to ''~/.m2/repository'', if nothing has been specified or configured.'
            cand -r 'Selects released artifacts only'
            cand --releases 'Selects released artifacts only'
            cand -s 'Selects snapshot artifacts only'
            cand --snapshots 'Selects snapshot artifacts only'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
            cand keep 'Keeps the artifacts matched by the filters and removes the rest'
            cand rm 'Removes the artifacts matched by the filters and keeps the rest'
            cand du 'Analyzes the size of the artifacts selected by the filters'
            cand help 'Prints this message or the help of the given subcommand(s)'
        }
        &'mcup;keep'= {
            cand -d 'Does not remove artifacts'
            cand --dry-run 'Does not remove artifacts'
            cand --list 'Prints the full path to the artifacts that will be removed'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
        }
        &'mcup;rm'= {
            cand -d 'Does not remove artifacts'
            cand --dry-run 'Does not remove artifacts'
            cand --list 'Prints the full path to the artifacts that will be removed'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
        }
        &'mcup;du'= {
            cand -o 'Defines whether (g)roups, (a)rtifacts and (v)ersions are included in the usage summary'
            cand --output 'Defines whether (g)roups, (a)rtifacts and (v)ersions are included in the usage summary'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
        }
        &'mcup;help'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
    ]
    $completions[$command]
}
