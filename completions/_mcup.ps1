
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'mcup' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'mcup'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-')) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'mcup' {
            [CompletionResult]::new('-g', 'g', [CompletionResultType]::ParameterName, 'Selects artifacts based on the group ID. Subgroups are included by default.')
            [CompletionResult]::new('--groups', 'groups', [CompletionResultType]::ParameterName, 'Selects artifacts based on the group ID. Subgroups are included by default.')
            [CompletionResult]::new('-a', 'a', [CompletionResultType]::ParameterName, 'Selects artifacts based on the artifact ID. Supports globbing like in ''maven-*-plugin''.')
            [CompletionResult]::new('--artifacts', 'artifacts', [CompletionResultType]::ParameterName, 'Selects artifacts based on the artifact ID. Supports globbing like in ''maven-*-plugin''.')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Selects artifacts based on version (ranges). Use ''<n>..'' to select the n most recent versions, ''..<n>'' to select the n oldest versions and ''<version>'' to select one specific version only.')
            [CompletionResult]::new('--versions', 'versions', [CompletionResultType]::ParameterName, 'Selects artifacts based on version (ranges). Use ''<n>..'' to select the n most recent versions, ''..<n>'' to select the n oldest versions and ''<version>'' to select one specific version only.')
            [CompletionResult]::new('-l', 'l', [CompletionResultType]::ParameterName, 'Sets the location of the local maven repository. Respects the directory configured in ''~/.m2/settings.xml''. Falls back to ''~/.m2/repository'', if nothing has been specified or configured.')
            [CompletionResult]::new('--local-repository', 'local-repository', [CompletionResultType]::ParameterName, 'Sets the location of the local maven repository. Respects the directory configured in ''~/.m2/settings.xml''. Falls back to ''~/.m2/repository'', if nothing has been specified or configured.')
            [CompletionResult]::new('-r', 'r', [CompletionResultType]::ParameterName, 'Selects released artifacts only')
            [CompletionResult]::new('--releases', 'releases', [CompletionResultType]::ParameterName, 'Selects released artifacts only')
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 'Selects snapshot artifacts only')
            [CompletionResult]::new('--snapshots', 'snapshots', [CompletionResultType]::ParameterName, 'Selects snapshot artifacts only')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('keep', 'keep', [CompletionResultType]::ParameterValue, 'Keeps the artifacts matched by the filters and removes the rest')
            [CompletionResult]::new('rm', 'rm', [CompletionResultType]::ParameterValue, 'Removes the artifacts matched by the filters and keeps the rest')
            [CompletionResult]::new('du', 'du', [CompletionResultType]::ParameterValue, 'Analyzes the size of the artifacts selected by the filters')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Prints this message or the help of the given subcommand(s)')
            break
        }
        'mcup;keep' {
            [CompletionResult]::new('-d', 'd', [CompletionResultType]::ParameterName, 'Does not remove artifacts')
            [CompletionResult]::new('--dry-run', 'dry-run', [CompletionResultType]::ParameterName, 'Does not remove artifacts')
            [CompletionResult]::new('--list', 'list', [CompletionResultType]::ParameterName, 'Prints the full path to the artifacts that will be removed')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            break
        }
        'mcup;rm' {
            [CompletionResult]::new('-d', 'd', [CompletionResultType]::ParameterName, 'Does not remove artifacts')
            [CompletionResult]::new('--dry-run', 'dry-run', [CompletionResultType]::ParameterName, 'Does not remove artifacts')
            [CompletionResult]::new('--list', 'list', [CompletionResultType]::ParameterName, 'Prints the full path to the artifacts that will be removed')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            break
        }
        'mcup;du' {
            [CompletionResult]::new('-o', 'o', [CompletionResultType]::ParameterName, 'Defines whether (g)roups, (a)rtifacts and (v)ersions are included in the usage summary')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'Defines whether (g)roups, (a)rtifacts and (v)ersions are included in the usage summary')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            break
        }
        'mcup;help' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
