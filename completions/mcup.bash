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
            
            du)
                cmd+="__du"
                ;;
            help)
                cmd+="__help"
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
            opts=" -g -a -v -l -r -s -h -V  --groups --artifacts --versions --local-repository --releases --snapshots --help --version  keep rm du help"
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
        
        mcup__du)
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
        mcup__help)
            opts=" -h -V  --help --version  "
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
        mcup__keep)
            opts=" -d -h  --dry-run --list --help  "
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
            opts=" -d -h  --dry-run --list --help  "
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
