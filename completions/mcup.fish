# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_mcup_global_optspecs
	string join \n g/groups= a/artifacts= v/versions= l/local-repository= r/releases s/snapshots h/help V/version
end

function __fish_mcup_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_mcup_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_mcup_using_subcommand
	set -l cmd (__fish_mcup_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c mcup -n "__fish_mcup_needs_command" -s g -l groups -d 'Selects artifacts based on the group ID. Subgroups are included by default.' -r
complete -c mcup -n "__fish_mcup_needs_command" -s a -l artifacts -d 'Selects artifacts based on the artifact ID. Supports globbing like in \'maven-*-plugin\'.' -r
complete -c mcup -n "__fish_mcup_needs_command" -s v -l versions -d 'Selects artifacts based on version (ranges). Use \'<n>..\' to select the n most recent versions, \'..<n>\' to select the n oldest versions and \'<version>\' to select one specific version only.' -r
complete -c mcup -n "__fish_mcup_needs_command" -s l -l local-repository -d 'Sets the location of the local maven repository. Respects the directory configured in \'~/.m2/settings.xml\'. Falls back to \'~/.m2/repository\', if nothing has been specified or configured.' -r
complete -c mcup -n "__fish_mcup_needs_command" -s r -l releases -d 'Selects released artifacts only'
complete -c mcup -n "__fish_mcup_needs_command" -s s -l snapshots -d 'Selects snapshot artifacts only'
complete -c mcup -n "__fish_mcup_needs_command" -s h -l help -d 'Print help'
complete -c mcup -n "__fish_mcup_needs_command" -s V -l version -d 'Print version'
complete -c mcup -n "__fish_mcup_needs_command" -f -a "keep" -d 'Keeps the artifacts matched by the filters and removes the rest'
complete -c mcup -n "__fish_mcup_needs_command" -f -a "rm" -d 'Removes the artifacts matched by the filters and keeps the rest'
complete -c mcup -n "__fish_mcup_needs_command" -f -a "du" -d 'Analyzes the size of the artifacts selected by the filters'
complete -c mcup -n "__fish_mcup_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c mcup -n "__fish_mcup_using_subcommand keep" -s d -l dry-run -d 'Does not remove artifacts'
complete -c mcup -n "__fish_mcup_using_subcommand keep" -l list -d 'Prints the full path to the artifacts that will be removed'
complete -c mcup -n "__fish_mcup_using_subcommand keep" -s h -l help -d 'Print help'
complete -c mcup -n "__fish_mcup_using_subcommand keep" -s V -l version -d 'Print version'
complete -c mcup -n "__fish_mcup_using_subcommand rm" -s d -l dry-run -d 'Does not remove artifacts'
complete -c mcup -n "__fish_mcup_using_subcommand rm" -l list -d 'Prints the full path to the artifacts that will be removed'
complete -c mcup -n "__fish_mcup_using_subcommand rm" -s h -l help -d 'Print help'
complete -c mcup -n "__fish_mcup_using_subcommand rm" -s V -l version -d 'Print version'
complete -c mcup -n "__fish_mcup_using_subcommand du" -s o -l output -d 'Defines whether (g)roups, (a)rtifacts and (v)ersions are included in the usage summary' -r
complete -c mcup -n "__fish_mcup_using_subcommand du" -s h -l help -d 'Print help'
complete -c mcup -n "__fish_mcup_using_subcommand du" -s V -l version -d 'Print version'
complete -c mcup -n "__fish_mcup_using_subcommand help; and not __fish_seen_subcommand_from keep rm du help" -f -a "keep" -d 'Keeps the artifacts matched by the filters and removes the rest'
complete -c mcup -n "__fish_mcup_using_subcommand help; and not __fish_seen_subcommand_from keep rm du help" -f -a "rm" -d 'Removes the artifacts matched by the filters and keeps the rest'
complete -c mcup -n "__fish_mcup_using_subcommand help; and not __fish_seen_subcommand_from keep rm du help" -f -a "du" -d 'Analyzes the size of the artifacts selected by the filters'
complete -c mcup -n "__fish_mcup_using_subcommand help; and not __fish_seen_subcommand_from keep rm du help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
