_mcup() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            mcup)
                cmd="mcup"
                ;;
            
            keep)
                cmd+="__keep"
                ;;
            rm)
                cmd+="__rm"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        mcup)
            opts=" -g -a -v -r -s -d -l -h -V  --groups --artifacts --versions --releases --snapshots --dry-run --list --local-repository --help --version  keep rm"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                --groups)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -g)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --artifacts)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -a)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --versions)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -v)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --local-repository)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        
        mcup__keep)
            opts=" -h  --help  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        mcup__rm)
            opts=" -h  --help  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

complete -F _mcup -o bashdefault -o default mcup
